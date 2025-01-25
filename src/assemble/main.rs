use image::GenericImageView;
use libheif_rs::{Channel, ColorSpace, CompressionFormat, EncoderQuality, HeifContext, Image, LibHeif, RgbChroma};


fn encode2() -> libheif_rs::Result<()> {
    let source = image::open("output/win98_00_00.png").unwrap();

    let width = source.width();
    let height = source.height();

    let mut image = Image::new(
        width,
        height,
        // TODO check the colorspace here, we probably want RGBA or smth
        ColorSpace::Rgb(RgbChroma::C444)
    )?;

    image.create_plane(Channel::R, width, height, 8)?;
    image.create_plane(Channel::G, width, height, 8)?;
    image.create_plane(Channel::B, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_r = planes.r.unwrap();
    let stride = plane_r.stride;

    let data_r = plane_r.data;
    let data_g = planes.g.unwrap().data;
    let data_b = planes.b.unwrap().data;

    for y in 0..height {
        let mut pixel_index = stride * y as usize;
        for x in 0..width {
            let [r,g,b,_a] = source.get_pixel(x, y).0;
            data_r[pixel_index] = r;
            data_g[pixel_index] = g;
            data_b[pixel_index] = b;
            pixel_index += 1;
        }
    }

    // Encode image and save it into file.
    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(
        CompressionFormat::Av1,
    )?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    context.encode_image(&image, &mut encoder, None)?;

    context.write_to_file("output.heif")?;

    Ok(())
}

fn main() {

    println!("Assembling!");

    encode2().unwrap();

}
