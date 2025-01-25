use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use photon_rs::{helpers, PhotonImage};
use rusttype::{Font, Scale};

pub fn render_text(photon_img: &PhotonImage, x: u32, y: u32, text: &str) -> PhotonImage {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    // OK this is mega hacky - it's a modified copy-paste of the code from photon, but
    // with the font replaced and some hacks.
    // Things I learned:
    // - it's crucial that we use a bitmap font for this, or to make it look convincingly like a
    //   bitmap by rendering at a lower DPI without antialiasing / hinting whatever
    // - I should probalby take a pass through and ensure I understand what all this code is doing.
    let font = Vec::from(include_bytes!("../res/ms-sans-serif.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let height = 26f32;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    draw_text_mut(
        &mut image,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        x,
        y - 4,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    let w = dynimage.width();
    let h = dynimage.height();
    PhotonImage::new(dynimage.into_bytes(), w, h)
}
