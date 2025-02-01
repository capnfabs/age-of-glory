use std::default;

use photon_rs::native::{open_image_from_bytes, save_image};
use photon_rs::{multiple, transform, PhotonImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use lazy_static::lazy_static;

fn render_taskbar(
    mut img: &mut PhotonImage,
) {
    let heights = [
        TASKBAR_LEFT.get_height(),
        TASKBAR_RIGHT.get_height(),
        TASKBAR_MID.get_height(),
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

    // 1: draw the taskbar_left
    let img_height = img.get_height();
    let mut x_pos = 0;
    let start_right = img.get_width() - TASKBAR_RIGHT.get_width();
    let taskbar_mid_width = TASKBAR_MID.get_width();
    let taskbar_y = img_height - taskbar_height;

    multiple::watermark(&mut img, &TASKBAR_LEFT, x_pos, taskbar_y);
    x_pos += TASKBAR_LEFT.get_width();

    // 2: fill the mid
    loop {
        multiple::watermark(&mut img, &TASKBAR_MID, x_pos, taskbar_y);

        x_pos += taskbar_mid_width;
        if x_pos >= start_right {
            break;
        }
    }

    // 3: draw the right
    multiple::watermark(&mut img, &TASKBAR_RIGHT, start_right, taskbar_y);
}

fn bytes_for_digit(digit: u8) -> &'static[u8] {
    match digit {
        0 => include_bytes!("../../res/0.png"),
        1 => include_bytes!("../../res/1.png"),
        2 => include_bytes!("../../res/2.png"),
        3 => include_bytes!("../../res/3.png"),
        4 => include_bytes!("../../res/4.png"),
        5 => include_bytes!("../../res/5.png"),
        6 => include_bytes!("../../res/6.png"),
        7 => include_bytes!("../../res/7.png"),
        8 => include_bytes!("../../res/8.png"),
        9 => include_bytes!("../../res/9.png"),
        _ => unimplemented!("that ain't a digit")
    }
}

macro_rules! load_res_image {
    ($string:literal) => {
        open_image_from_bytes(include_bytes!(concat!("../../res/",$string))).expect(concat!("Couldn't open pre-baked img ", $string))
    }
}

fn load_digits() -> Vec<PhotonImage> {
    let mut result = vec![];
    let mut digit = 0;
    while digit <= 9 {
        let img = open_image_from_bytes(bytes_for_digit(digit)).expect("Should be able to open pre-baked image");
        let img = scale(&img);
        result.push(img);
        digit += 1;
    };
    result
}

lazy_static! {
    static ref DIGITS: Vec<PhotonImage> = load_digits();

    static ref SYMBOL_COLON: PhotonImage = scale(&load_res_image!("colon.png"));
    static ref SYMBOL_AM: PhotonImage = scale(&load_res_image!("am.png"));
    static ref SYMBOL_PM: PhotonImage = scale(&load_res_image!("pm.png"));

    static ref TASKBAR_LEFT: PhotonImage = scale(&load_res_image!("taskbar-start.png"));
    static ref TASKBAR_MID: PhotonImage = scale(&load_res_image!("taskbar-mid.png"));
    static ref TASKBAR_RIGHT: PhotonImage = scale(&load_res_image!("taskbar-end.png"));
}


fn render_clock(mut img: &mut PhotonImage, clock_pos: (u32, u32), hour: u8, min: u8) {
    assert!(hour < 24);
    assert!(min < 60);

    let pm  = (hour / 12) != 0;
    let two_digit_hour = (hour % 12) == 0 || (hour % 12) == 11;

    let mut x_pos = clock_pos.0;
    let y_pos = clock_pos.1;
    if two_digit_hour {
        // Shift left by three pixels (unscaled) if there's a second digit
        x_pos -= 3 * TASKBAR_SCALE;
    }
    if two_digit_hour {
        // it's always a one
        multiple::watermark(&mut img, &DIGITS[1], x_pos, y_pos);
        x_pos += DIGITS[1].get_width() + TASKBAR_SCALE;
    }

    let second_digit = if hour == 12 || hour == 0 {
        2
    } else {
        (hour % 12) % 10
    };
    let digit_img = &DIGITS[second_digit as usize];
    multiple::watermark(&mut img, &digit_img, x_pos, y_pos);
    x_pos += digit_img.get_width() + TASKBAR_SCALE;

    multiple::watermark(&mut img, &SYMBOL_COLON, x_pos, y_pos);
    x_pos += SYMBOL_COLON.get_width() + TASKBAR_SCALE;

    let digit_img = &DIGITS[(min / 10) as usize];

    multiple::watermark(&mut img, &digit_img, x_pos, y_pos);
    x_pos += digit_img.get_width() + TASKBAR_SCALE;

    let digit_img = &DIGITS[(min % 10) as usize];
    multiple::watermark(&mut img, &digit_img, x_pos, y_pos);
    x_pos += digit_img.get_width() + TASKBAR_SCALE;

    // AM / PM
    x_pos += 2 * TASKBAR_SCALE;

    if pm {
        multiple::watermark(&mut img, &SYMBOL_PM, x_pos, y_pos);
    } else {
        multiple::watermark(&mut img, &SYMBOL_AM, x_pos, y_pos);
    }

}

fn scale(img: &PhotonImage) -> PhotonImage {
    transform::resize(img, img.get_width() * TASKBAR_SCALE, img.get_height() * TASKBAR_SCALE, transform::SamplingFilter::Nearest)
}

const TASKBAR_SCALE: u32 = 2;

fn main() {
    let bkg_png = include_bytes!("../../res/bkg.png");
    // relative to taskbar_right's origin, prior to scaling
    let clock_offset = (54, 11);

    let mut img = open_image_from_bytes(bkg_png).expect("File should open");
    let clock_offset = (clock_offset.0 * TASKBAR_SCALE, clock_offset.1 * TASKBAR_SCALE);

    let taskbar_right_offset_x = img.get_width() - TASKBAR_RIGHT.get_width();
    let taskbar_right_offset_y = img.get_height() - TASKBAR_RIGHT.get_height();
    let clock_pos = (taskbar_right_offset_x + clock_offset.0, taskbar_right_offset_y + clock_offset.1);

    render_taskbar(&mut img);
    render_clock(&mut img, clock_pos, 13, 00);

    save_image(img, "single.png").expect("Save failed");

    // (0..24).into_par_iter().for_each(|hour| {
    //     for min in 0..60 {
    //         let am_or_pm = if hour < 12 {
    //             "AM"
    //         } else {
    //             "PM"
    //         };
    //         let hour_display = if hour == 0 {
    //             12
    //         } else {
    //             hour
    //         };

    //         let timestr = format!("{:>2}:{:02} {}", hour_display, min, am_or_pm);

    //         let new_img = render_clock::render_text(&bkg_img, taskbar_right_start + clock_offset.0, taskbar_top + clock_offset.1, &timestr);
    //         let filename = format!("output/win98_{:02}_{:02}.png", hour, min);
    //         save_image(new_img, &filename).expect("Save failed");
    //     }
    // });

    println!("Done!");
}
