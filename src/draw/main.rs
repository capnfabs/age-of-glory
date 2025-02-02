mod render_taskbar;
mod make_listing;

use std::path::PathBuf;

use itertools::Itertools;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use clap::{arg, Parser, Subcommand};



/// Draws a Win98 taskbar on a background image. Hell yeah.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The source file to use as the background image
    #[arg(short, long, value_name = "FILE")]
    source: PathBuf,

    /// The output destination. Use a file / folder, based on the mode.
    /// Will create a folder if it doesn't exist already.
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Operates on a single image
    Single {
        /// Set the time of the clock, HH:mm format
        clock_time: String,
    },
    /// Generates a whole day's worth of timestamps to a folder
    Batch {
    }
}

fn render_batch(base_img: &PhotonImage, output_directory: &std::path::Path) {
    (0..24).into_par_iter().for_each(|hour| {
        for min in 0..60 {
            let mut new_img = base_img.clone();
            render_taskbar::render_taskbar(&mut new_img, hour, min);
            // TODO directory
            let file_path = output_directory.join(format!("win98_{:02}_{:02}.png", hour, min));
            let filename = file_path.to_str().expect("Unicode something something");
            save_image(new_img, &filename).expect("Save failed");
        }
    });
    let listing_path = output_directory.join("listing.json");
    make_listing::write_listing(listing_path.to_str().expect("Unicode something something"));
}

fn main() {
    let cli = Cli::parse();

    let mut img = open_image(cli.source.to_str().expect("idk unicode something")).expect("File should open");
    match cli.command {
        Commands::Single { clock_time } => {

            let output = cli.output.map(|path| path.into_os_string().into_string().expect("Something weird about unicode that I don't fully understand")).unwrap_or("output.png".to_string());

            // TODO make this nice, we can probably collect all the errors into one thing
            let (hour, min) = clock_time.split(':').map(|n| n.parse::<u8>().expect("Time format must be HH:mm")).collect_tuple().expect("Time format must be HH:mm");
            render_taskbar::render_taskbar(&mut img, hour, min);
            // TODO store this in output correctly
            save_image(img, &output).expect("Save failed");
        },
        Commands::Batch {  } => {
            let output = cli.output.unwrap_or("output".into());
            std::fs::create_dir_all(&output).expect("Couldn't create output directory");
            render_batch(&img, &output);
        },
    }
    println!("Done!");
}
