use std::{env, fs, time::Duration};

use anyhow::{Ok, bail};
use regex::Regex;

#[derive(Debug)]
pub struct FileMetadata {
    pub file_path: FileTag,
    pub title: FileTag,
    pub performer: FileTag,
    pub date: FileTag,
    pub genre: FileTag,
}

#[derive(Debug)]
pub enum FileTag {
    FILE(String),
    TITLE(String),
    PERFORMER(String),
    DATE(u16),
    GENRE(String),
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

impl FileTag {
    fn read(tag: &FileTag, source: &str) -> anyhow::Result<Self> {
        match tag {
            FileTag::FILE(_) => return Ok(Self::new_file(source)?),
            _ => return Ok(Self::new_file(source)?),
        }
    }

    fn new_file(source: &str) -> anyhow::Result<FileTag> {
        for line in source.lines() {
            if let Some(tag) = line.split_whitespace().nth(0) {
                if tag == "FILE" {
                    let regex = Regex::new(r#""(.*?)""#)?;
                    if let Some(r) = regex.captures(line) {
                        let res = r.get(1).unwrap().as_str().to_string();
                        return Ok(FileTag::FILE(res));
                    }
                }
            }
        }
        bail!("Can`t to read file path")
    }

    fn new_title(source: &str) -> anyhow::Result<FileTag> {
        for line in source.lines() {
            if let Some(tag) = line.split_whitespace().nth(0) {
                if tag == "TITLE" {
                    let regex = Regex::new(r#""(.*?)""#)?;
                    if let Some(r) = regex.captures(line) {
                        let res = r.get(1).unwrap().as_str().to_string();
                        return Ok(FileTag::TITLE(res));
                    }
                }
            }
        }
        bail!("Can`t to read file title")
    }

    fn new_performer(source: &str) -> anyhow::Result<FileTag> {
        for line in source.lines() {
            if let Some(tag) = line.split_whitespace().nth(0) {
                if tag == "PERFORMER" {
                    let regex = Regex::new(r#""(.*?)""#)?;
                    if let Some(r) = regex.captures(line) {
                        let res = r.get(1).unwrap().as_str().to_string();
                        return Ok(FileTag::PERFORMER(res));
                    }
                }
            }
        }
        bail!("Can`t to read file performer")
    }

    fn new_date(source: &str) -> anyhow::Result<FileTag> {
        for line in source.lines() {
            if let Some(tag) = line.split_whitespace().nth(1) {
                if tag == "DATE" {
                    return Ok(FileTag::DATE(
                        line.split_whitespace().nth(2).unwrap().parse::<u16>()?,
                    ));
                }
            }
        }
        bail!("Can`t to read file date")
    }

    fn new_genre(source: &str) -> anyhow::Result<FileTag> {
        for line in source.lines() {
            if let Some(tag) = line.split_whitespace().nth(1) {
                if tag == "GENRE" {
                    let regex = Regex::new(r#""(.*?)""#)?;
                    if let Some(r) = regex.captures(line) {
                        let res = r.get(1).unwrap().as_str().to_string();
                        return Ok(FileTag::GENRE(res));
                    }
                }
            }
        }
        bail!("Can`t to read file genre")
    }

    fn unwrap_file(&self) -> String {
        match self {
            FileTag::FILE(file) => file.to_string(),
            _ => {
                panic!("paniced when unwrap file tag")
            }
        }
    }

    fn unwrap_title(&self) -> String {
        match self {
            FileTag::TITLE(title) => title.to_string(),
            _ => {
                panic!("paniced when unwrap title tag")
            }
        }
    }

    fn unwrap_performer(&self) -> String {
        match self {
            FileTag::PERFORMER(performer) => performer.to_string(),
            _ => {
                panic!("paniced when unwrap performer tag")
            }
        }
    }

    fn unwrap_date(&self) -> String {
        match self {
            FileTag::DATE(date) => date.to_string(),
            _ => {
                panic!("paniced when unwrap date tag")
            }
        }
    }

    fn unwrap_genre(&self) -> String {
        match self {
            FileTag::GENRE(genre) => genre.to_string(),
            _ => {
                panic!("paniced when unwrap genre tag")
            }
        }
    }
}

#[test]
fn test_read_file_metadata() -> anyhow::Result<()> {
    let file_path = env::var("FILE_PATH_CUE")?;

    let text = read_file(&file_path)?;

    let path = FileTag::new_file(&read_file_metadata(&text)?)?.unwrap_file();
    let title = FileTag::new_title(&read_file_metadata(&text)?)?.unwrap_title();
    let performer = FileTag::new_performer(&read_file_metadata(&text)?)?.unwrap_performer();
    let date = FileTag::new_date(&read_file_metadata(&text)?)?.unwrap_date();
    let genre = FileTag::new_genre(&read_file_metadata(&text)?)?.unwrap_genre();

    println!("{}", path);
    println!("{}", title);
    println!("{}", performer);
    println!("{}", date);
    println!("{}", genre);

    Ok(())
}
