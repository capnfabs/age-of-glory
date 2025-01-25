use photon_rs::multiple;
use photon_rs::native::{open_image_from_bytes, save_image};

fn main() {
    let bkg_png = include_bytes!("../res/bkg.png");
    let taskbar_left_bytes = include_bytes!("../res/taskbar-left.png");
    let taskbar_right_bytes = include_bytes!("../res/taskbar-right.png");
    let taskbar_mid_bytes = include_bytes!("../res/taskbar-mid.png");

    let mut img = open_image_from_bytes(bkg_png).expect("File should open");
    let taskbar_left = open_image_from_bytes(taskbar_left_bytes).expect("File should open");
    let taskbar_mid = open_image_from_bytes(taskbar_mid_bytes).expect("File should open");
    let taskbar_right = open_image_from_bytes(taskbar_right_bytes).expect("File should open");

    let heights = [
        taskbar_left.get_height(),
        taskbar_right.get_height(),
        taskbar_mid.get_height(),
    ];
    let taskbar_height = heights.iter().reduce(|a, b| {
        if a != b {
            panic!("Expected all heights to be the same")
        }
        b
    }).unwrap();

    // 1: drop the taskbar_left

    let img_height = img.get_height();
    let mut x_pos = 0;
    let start_right = img.get_width() - taskbar_right.get_width();
    let taskbar_mid_width = taskbar_mid.get_width();
    let taskbar_y = img_height - taskbar_height;

    multiple::watermark(
        &mut img,
        &taskbar_left,
        x_pos,
        taskbar_y,
    );
    x_pos += taskbar_left.get_width();

    // 2: fill the mid
    loop {
        multiple::watermark(
            &mut img,
            &taskbar_mid,
            x_pos,
            taskbar_y,
        );

        x_pos += taskbar_mid_width;
        if x_pos >= start_right {
            break;
        }
    }

    // 3: draw the right
    multiple::watermark(&mut img, &taskbar_right, start_right, taskbar_y);

    save_image(img,"manipulated_image.jpg").expect("Save failed");

    println!("Done!");
}
