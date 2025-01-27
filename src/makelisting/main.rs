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


fn write_listing() {
    let mut entries = vec![];
    // now emit JSON file
    // TODO fix timezones, we'd ideally use the current timezone here
    // TODO we'd also ideally assert that this isn't a day where the timezone
    // changes
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
    println!("Writing listing...");
    write_listing();
    println!("Done!");
}
