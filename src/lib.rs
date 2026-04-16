use std::{env, fs, time::Duration};

use anyhow::{Ok, bail};
use regex::Regex;

#[derive(Debug)]
pub struct FileMetadata {
    pub file_path: String,
    pub title: String,
    pub performer: String,
    pub date: u16,
    pub genre: String,
}

pub struct TrackMetadata {
    pub file_path: String,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub age: u16,
    pub start_time: Duration,
    pub end_time: Duration,
}

fn read_file(file_path: &str) -> anyhow::Result<String> {
    // I don't know why I omitted this little thing
    Ok(fs::read_to_string(file_path)?)
}

fn read_file_metadata(text: &str) -> anyhow::Result<String> {
    if let Some((h, _)) = text.split_once("TRACK") {
        let header = h.trim_end();
        return Ok(header.to_string());
    }

    bail!("Can`t to read file metadata")
}

fn parse_file_metadata(text: &str) -> anyhow::Result<FileMetadata> {
    let mut file_path = String::new();
    let mut title = String::new();
    let mut performer = String::new();
    let mut date = 0;
    let mut genre = String::new();

    for line in text.lines() {
        if let Some(first_word) = line.split_whitespace().next() {
            match first_word {
                "FILE" => {
                    let regex = Regex::new(r#""(.*?)""#).unwrap();
                    if let Some(caps) = regex.captures(line) {
                        file_path = caps.get(1).unwrap().as_str().to_string();
                    }
                }
                "TITLE" => {
                    let regex = Regex::new(r#""(.*?)""#).unwrap();
                    if let Some(caps) = regex.captures(line) {
                        title = caps.get(1).unwrap().as_str().to_string();
                    }
                }
                "PERFORMER" => {
                    let regex = Regex::new(r#""(.*?)""#).unwrap();
                    if let Some(caps) = regex.captures(line) {
                        performer = caps.get(1).unwrap().as_str().to_string();
                    }
                }
                _ => {}
            }
        } else {
            bail!("Can`t to parse file metadata")
        }

        if let Some(first_word) = line.split_whitespace().next() {
            match first_word {
                "REM" => {
                    let mut words = line.split_whitespace();
                    if let Some(second_word) = words.nth(1) {
                        match second_word {
                            "DATE" => {
                                if let Some(d) = words.nth(2) {
                                    date = d.parse::<u16>().unwrap();
                                }
                            }
                            "GENRE" => {
                                let regex = Regex::new(r#""(.*?)""#).unwrap();
                                if let Some(caps) = regex.captures(line) {
                                    genre = caps.get(1).unwrap().as_str().to_string();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        } else {
            bail!("Can`t to parse file metadata")
        }
    }

    Ok(FileMetadata {
        file_path,
        title,
        performer,
        date,
        genre,
    })
}

#[test]
fn test_read_file_metadata() -> anyhow::Result<()> {
    let file_path = env::var("FILE_PATH_CUE")?;

    let text = read_file(&file_path)?;

    let file_metadata = parse_file_metadata(&read_file_metadata(&text)?)?;

    println!("{:#?}", file_metadata);
    //println!("{}", read_file_metadata(&text)?);
    Ok(())
}
