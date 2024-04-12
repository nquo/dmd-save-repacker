use std::io::Error;
use std::{fs::File, path::Path};

use flate2::{
    read::{DeflateDecoder, DeflateEncoder},
    Compression,
};

#[derive(Debug)]
pub enum OutFileType {
    Zip,
    Json,
}

#[derive(Debug)]
pub struct Config {
    pub out_file_type: OutFileType,
    pub in_file_path: String,
    pub out_file_path: String,
}

static R_RESULT: &str = "result";
static R_JSON: &str = "json";
static R_SAV: &str = "sav";

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let in_file_path = args[1].clone();
        let type_cfg = Self::get_type(&in_file_path);

        if let Err(e) = type_cfg {
            return Err(e);
        }
        let action = type_cfg.unwrap();

        let mut out_file_path = format!("{}.{}", String::from(R_RESULT), Self::get_ext(&action));

        if args.len() > 2 {
            out_file_path = args[2].clone();
        }

        Ok(Config {
            out_file_type: action,
            in_file_path,
            out_file_path,
        })
    }

    fn get_type(filename: &str) -> Result<OutFileType, &'static str> {
        let (_, ext) = split_file_name(filename);
        match ext.as_str() {
            "sav" => Ok(OutFileType::Json),
            "json" => Ok(OutFileType::Zip),
            _ => Err("Invalid file extension. Expected .sav or .json"),
        }
    }

    fn get_ext(ext: &OutFileType) -> &'static str {
        match ext {
            OutFileType::Zip => R_SAV,
            OutFileType::Json => R_JSON,
        }
    }
}

fn split_file_name(file_name: &str) -> (String, String) {
    let path = Path::new(file_name);
    let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
    let file_ext = path.extension().unwrap().to_str().unwrap().to_string();

    (file_stem, file_ext)
}

fn get_allowed_filename(file_name: &String) -> String {
    let mut suffix = 0;
    let (name, ext) = split_file_name(&file_name);
    let mut new_file_name = get_new_name(&name, suffix, &ext);
    while Path::new(&new_file_name).exists() {
        suffix += 1;
        new_file_name = get_new_name(&name, suffix, &ext);
    }
    new_file_name
}

fn get_new_name(name: &String, suffix: i32, ext: &String) -> String {
    let suffix_str = format!("{:02}", suffix);
    format!("{}_{}.{}", name, suffix_str, ext)
}

fn packunpack(input: &str, out: &str, out_type: OutFileType) -> Result<(), Error> {
    let input_file = File::open(input)?;
    let mut output_file = File::create_new(out)?;

    match out_type {
        OutFileType::Zip => {
            let mut encoder = DeflateEncoder::new(input_file, Compression::default());
            std::io::copy(&mut encoder, &mut output_file)?;
            println!("{} successfully created", out);
        }
        OutFileType::Json => {
            let mut decoder = DeflateDecoder::new(input_file);
            std::io::copy(&mut decoder, &mut output_file)?;
            println!("successfully unpacked to {}", out);
        }
    };

    Ok(())
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let input = &config.in_file_path;
    let mut output = &config.out_file_path;

    let tmp = get_allowed_filename(&output);
    output = &tmp;

    if let Err(err) = packunpack(input, output, config.out_file_type) {
        println!("{}", err);
    }

    Ok(())
}
