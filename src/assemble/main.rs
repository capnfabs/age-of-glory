use std::io;

use base64::prelude::*;
use image::GenericImageView;
use libheif_rs::{
    Channel, ColorSpace, CompressionFormat, EncoderQuality, HeifContext, Image, ItemId, LibHeif,
    RgbChroma,
};
use plist::Value;
use serde::Serialize;
use chrono::prelude::*;
use std::fs::File;

#[derive(Serialize)]
struct TimeEntry {
    #[serde(rename = "fileName")]
    file_name: String,
    #[serde(rename = "isPrimary")]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    is_primary: bool,
    time: String
}

fn frame_from_path(path: &str) -> libheif_rs::Result<Image> {
    let source = image::open(&path).unwrap();

    let width = source.width();
    let height = source.height();

    let mut image = Image::new(
        width,
        height,
        // TODO check the colorspace here, we probably want RGBA or smth
        ColorSpace::Rgb(RgbChroma::C444),
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
            let [r, g, b, _a] = source.get_pixel(x, y).0;
            data_r[pixel_index] = r;
            data_g[pixel_index] = g;
            data_b[pixel_index] = b;
            pixel_index += 1;
        }
    }
    Ok(image)
}

#[derive(Serialize)]
struct CompleteMetadata {
    #[serde(rename = "ti")]
    entries: Vec<TimeMetadata>,
}

#[derive(Serialize)]
struct TimeMetadata {
    #[serde(rename = "i")]
    index: u32,
    #[serde(rename = "t")]
    time: f32,
}

const NUM_HOURS: u8 = 4;
const OUTPUT_FILENAME: &'static str = "output.heic";

fn build_metadata() -> Vec<u8> {
    let mut data = vec![];
    // TODO unhardcode this
    for i in 0..NUM_HOURS {
        data.push(TimeMetadata {
            index: 0,
            time: (i as f32) / 24f32,
        });
    }
    let complete = CompleteMetadata { entries: data };

    let mut bytes: Vec<u8> = vec![];
    plist::to_writer_binary(&mut bytes, &complete).unwrap();
    bytes
}

fn write_xmp_something_something_old(
    mut w: impl io::Write,
    plist_bytes: &[u8],
) -> Result<(), io::Error> {
    // TODO can we just use this constant? lol
    let id = "W5M0MpCehiHzreSzNTczkc9d";
    let plist_bytes_encoded = BASE64_STANDARD.encode(plist_bytes);
    write!(
        w,
        r#"<?xpacket begin="" id="{}"?>
    <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="XMP Core 6.0.0">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:Description rdf:about="" xmlns:apple_desktop="http://ns.apple.com/namespace/1.0/" apple_desktop:h24="{}"/>
        </rdf:RDF>
    </x:xmpmeta>
<?xpacket end="r"?>
"#,
        id, &plist_bytes_encoded
    )
}

fn write_xmp_something_something(
    mut w: impl io::Write,
    plist_bytes: &[u8],
) -> Result<(), io::Error> {
    let plist_bytes_encoded = BASE64_STANDARD.encode(plist_bytes);
    write!(
        w,
        r#"<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:apple_desktop="http://ns.apple.com/namespace/1.0/" apple_desktop:h24="{}"/></rdf:RDF>"#,
        &plist_bytes_encoded
    )
}

fn encode2() -> libheif_rs::Result<()> {
    // Encode image and save it into file.
    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    encoder.set_quality(EncoderQuality::LossLess)?;

    // metadata from here:
    // https://github.com/rlxone/Equinox/blob/main/EquinoxCore/EquinoxCore/Cores/MetadataCore.swift#L57

    // time is encoded as a fraction of the seconds since midnight I guess, e.g. for 1am it's 1/24 = 0.04166667
    // encoded as pairs of "i" (index, probably image index) and "t" (time)

    for hour in 0..NUM_HOURS {
        let path = format!("output/win98_{:02}_00.png", hour);
        let frame = frame_from_path(&path).unwrap();
        let image_handle = context.encode_image(&frame, &mut encoder, None)?;
        if hour == 0 {
            // add metadata
            let metadata = build_metadata();
            let mut full_metadata_packet = vec![];
            write_xmp_something_something_old(&mut full_metadata_packet, &metadata).unwrap();
            println!("{:}", std::str::from_utf8(&full_metadata_packet).unwrap());
            // let content_type = "application/rdf+xml";
            // let item_type = b"mime";
            // context.add_generic_metadata(
            //     &image_handle,
            //     &full_metadata_packet,
            //     item_type,
            //     Some(content_type),
            // )?;
            context.add_xmp_metadata(&image_handle, &full_metadata_packet)?;
        }
        println!("Done {}/24", hour + 1);
    }
    println!("Finalizing...");
    context.write_to_file(&OUTPUT_FILENAME)?;
    println!("Done!");

    Ok(())
}

fn check_existing_image(path: &str) -> libheif_rs::Result<()> {
    let ctx = HeifContext::read_from_file(path)?;
    println!("Has {} images", ctx.number_of_top_level_images());
    for (i, handle) in ctx.top_level_image_handles().iter().enumerate() {
        println!("Image #{}", i);
        println!("{}x{}", handle.width(), handle.height());
        let all_metadata = handle.all_metadata();
        for (item, metadata) in all_metadata.iter().enumerate() {
            println!("Item: {}", item);
            println!("item type: {}", metadata.item_type);
            println!("content type: {}", metadata.content_type);
            println!("uri type: {}", metadata.uri_type);
            println!(
                "raw_data: {}",
                std::str::from_utf8(&metadata.raw_data).unwrap()
            );
            let raw_data = std::str::from_utf8(&metadata.raw_data).unwrap();
            let start_str = "apple_desktop:h24=\"";
            let start_tag_pos = raw_data.find(start_str).unwrap();
            let start_pos = start_tag_pos + start_str.len();
            let end = raw_data[start_pos..].find('"').unwrap();
            let b64 = &raw_data[start_pos..(end + start_pos)];
            println!("base64: {}", b64);
            let bytes = BASE64_STANDARD.decode(b64).unwrap();
            let v = Value::from_reader(std::io::Cursor::new(&bytes)).unwrap();
            println!("v: {:#?}", v);
        }
    }
    Ok(())
}

fn write_listing() {
    let mut entries = vec![];
    // now emit JSON file
    // My computer is in GMT+1 but for some reason this needs to be GMT+2
    // I do not know why
    let tz = FixedOffset::east_opt(2*3600).unwrap();
    for hour in 0..24 {
        for min in 0..60 {
            let filename = format!("win98_{:02}_{:02}.png", hour, min);
            let timestamp: String = NaiveDateTime::new(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), NaiveTime::from_hms_opt(hour, min, 0).unwrap()).and_local_timezone(tz).unwrap().to_rfc3339();

            entries.push(TimeEntry {
                file_name: filename,
                is_primary: hour == 0 && min == 0,
                time: timestamp,
            });
        }
    }
    let mut file = File::create("output/listing.json").unwrap();

    serde_json::to_writer_pretty(&mut file, &entries).unwrap();

}

fn main() {
    //println!("Assembling!");
    //encode2().unwrap();

    //println!("My image:");
    //check_existing_image(&OUTPUT_FILENAME).unwrap();
    //println!("Wallpapper image:");
    //check_existing_image("output_wallpapper.heic").unwrap();
    // check_existing_image("/Users/fabian/Desktop/wallpaper.heic").unwrap();

    println!("Writing listing...");
    write_listing();
    println!("Done!");
    //encode2().unwrap();
}
