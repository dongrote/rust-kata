use std::io::{Error, Write};
use std::path::{PathBuf, Path};
use std::fs::{OpenOptions, File};

#[derive(Debug)]
pub struct Config {
    pub line: String,
    pub file: PathBuf,
}

impl Config {
  pub fn from(arguments: &Vec<String>) -> Result<Config, &'static str> {
    if arguments.len() < 3 {
      return Err("not enough command line arguments");
    }

    Ok(Config {
      line: arguments[1].clone(),
      file: PathBuf::from(arguments[2].clone()),
    })
  }
}

pub fn append(file: &Path, data: &str) -> Result<(), &'static str> {
  match open_file_for_append(file) {
    Ok(mut f) => {
      match f.write_all(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("there was an error writing to the file"),
      }
    },
    Err(_) => Err("there was an error opening the file"),
  }
}

pub fn append_line(file: &Path, data: &str) -> Result<(), &'static str> {
  append(file, &format!("{}\n", data))
}

fn open_file_for_append(file: &Path) -> Result<File, Error> {
  OpenOptions::new().append(true).create(true).open(file)
}
