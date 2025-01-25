mod render_clock;

use photon_rs::native::{open_image_from_bytes, save_image};
use photon_rs::{multiple, PhotonImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn render_taskbar(
    mut img: &mut PhotonImage,
    taskbar_left: &PhotonImage,
    taskbar_mid: &PhotonImage,
    taskbar_right: &PhotonImage,
) {
    let heights = [
        taskbar_left.get_height(),
        taskbar_right.get_height(),
        taskbar_mid.get_height(),
    ];
    let taskbar_height = heights
        .iter()
        .reduce(|a, b| {
            if a != b {
                panic!("Expected all heights to be the same")
            }
            b
        })
        .unwrap();

    // 1: drop the taskbar_left

    let img_height = img.get_height();
    let mut x_pos = 0;
    let start_right = img.get_width() - taskbar_right.get_width();
    let taskbar_mid_width = taskbar_mid.get_width();
    let taskbar_y = img_height - taskbar_height;

    multiple::watermark(&mut img, &taskbar_left, x_pos, taskbar_y);
    x_pos += taskbar_left.get_width();

    // 2: fill the mid
    loop {
        multiple::watermark(&mut img, &taskbar_mid, x_pos, taskbar_y);

        x_pos += taskbar_mid_width;
        if x_pos >= start_right {
            break;
        }
    }

    // 3: draw the right
    multiple::watermark(&mut img, &taskbar_right, start_right, taskbar_y);
}

fn main() {
    let bkg_png = include_bytes!("../../res/bkg.png");
    let taskbar_left_bytes = include_bytes!("../../res/taskbar-left.png");
    let taskbar_mid_bytes = include_bytes!("../../res/taskbar-mid.png");
    let taskbar_right_bytes = include_bytes!("../../res/taskbar-right.png");
    // relative to taskbar_right's origin
    let clock_offset = (71, 22);

    let mut img = open_image_from_bytes(bkg_png).expect("File should open");
    let taskbar_left = open_image_from_bytes(taskbar_left_bytes).expect("File should open");
    let taskbar_mid = open_image_from_bytes(taskbar_mid_bytes).expect("File should open");
    let taskbar_right = open_image_from_bytes(taskbar_right_bytes).expect("File should open");

    render_taskbar(&mut img, &taskbar_left, &taskbar_mid, &taskbar_right);

    let bkg_img = img;

    // TODO: this is calculated in multiple places, sort it out
    let taskbar_top = bkg_img.get_height() - taskbar_left.get_height();
    let taskbar_right_start = bkg_img.get_width() - taskbar_right.get_width();

    (0..=23).into_par_iter().for_each(|hour| {
        for min in 0..=59 {
            let am_or_pm = if hour < 12 {
                "AM"
            } else {
                "PM"
            };
            let hour_display = if hour == 0 {
                12
            } else {
                hour
            };

            let timestr = format!("{:>2}:{:02} {}", hour_display, min, am_or_pm);

            let new_img = render_clock::render_text(&bkg_img, taskbar_right_start + clock_offset.0, taskbar_top + clock_offset.1, &timestr);
            let filename = format!("output/win98_{:02}_{:02}.png", hour, min);
            save_image(new_img, &filename).expect("Save failed");
        }
    });

    println!("Done!");
}
