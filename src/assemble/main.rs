use image::GenericImageView;
use libheif_rs::{Channel, ColorSpace, CompressionFormat, EncoderQuality, HeifContext, Image, LibHeif, RgbChroma};

fn frame_from_path(path: &str) -> libheif_rs::Result<Image> {
    let source = image::open(&path).unwrap();

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
    Ok(image)
}

fn encode2() -> libheif_rs::Result<()> {
    // Encode image and save it into file.
    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(
        CompressionFormat::Av1,
    )?;
    encoder.set_quality(EncoderQuality::LossLess)?;

    // Metadata lives on image 0 / primary image, see
    // https://github.com/rlxone/Equinox/blob/43ad99abb45a027048a1bcd32d99a6b5468a234a/EquinoxCore/EquinoxCore/Cores/ImageCore.swift#L71
    // TODO: add metadata here, see context.add_{}_metadata()

    // metadata from here:
    // https://github.com/rlxone/Equinox/blob/main/EquinoxCore/EquinoxCore/Cores/MetadataCore.swift#L57

    // time is encoded as a fraction of the seconds since midnight I guess, e.g. for 1am it's 1/24 = 0.04166667
    // encoded as pairs of "i" (index, probably image index) and "t" (time)

    // unclear if we also have to encode an "appearance" (night / day)
    // and explicitly code somewhere that we care about time, i.e. ImageMetadata=ti
    // encoded as a property list, and then apparently converted to base64
    // and stuffed into a metadata field
    // and then there's a bunch of extra wrapping content
    // Can use https://crates.io/crates/plist to build the plist

    for hour in 0..24 {
        let path = format!("output/win98_{:02}_00.png", hour);
        let frame = frame_from_path(&path).unwrap();
        let imgHandle = context.encode_image(&frame, &mut encoder, None)?;
        println!("Done {}/24", hour + 1);
    }
    println!("Finalizing...");
    context.write_to_file("output.heif")?;
    println!("Done!");

    Ok(())
}

fn main() {

    println!("Assembling!");

    encode2().unwrap();

}
