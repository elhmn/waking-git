use crate::shmup;
use clap::Args;
use core::converters;
use std::fs::File;
use std::io::{Error, Read};

#[derive(Args, Debug)]
pub struct RunArgs {
    /// The json file containing the converted data
    #[clap(value_name = "FILE", index = 1)]
    file: Option<String>,
}

pub fn run(args: &RunArgs) {
    let file = args.file.clone().unwrap_or_default();

    //load the converted.json data from the file
    let converted_data = match load_converted_data(&file) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to load converted data: {err}");
            return;
        }
    };

    shmup::run(converted_data);
}

fn load_converted_data(file: &str) -> Result<core::converters::shmup::Data, Error> {
    let mut f = File::open(file)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    let data: converters::shmup::Data = serde_json::from_str(&data)?;
    Ok(data)
}
