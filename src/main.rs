use std::{error::Error, fs::read_dir, io};

use chrono::{NaiveDate, NaiveDateTime};

fn main() -> Result<(), Box<dyn Error>> {

    for file in read_dir("./res/recordings")?.filter_map(Result::ok).map(|entry| {
        return entry.file_name();
    }) {
        let filename = file
            .clone()
            .into_string()
            .map_err(|_| io::Error::other("Failed into_string"))?
            .strip_suffix(".WAV")
            .ok_or(io::Error::other("Failed to parse filename"))?
            .to_string();
        

        let date = NaiveDate::parse_from_str(filename.truncate(8), "%Y%m%d");

        println!("{:?} -> {:?} -> {:?}", file, filename, date);
    }

    return Ok(());
}