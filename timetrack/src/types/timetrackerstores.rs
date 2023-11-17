use std::path::PathBuf;
use std::{path::Path, ffi::OsString};
use std::io::{self, Read, Write};
use std::fs::OpenOptions;
use crate::ActiveDuration;

#[derive(Clone, Debug)]
pub struct TimeTrackerFileStore {
    file_path: OsString,
    duration: ActiveDuration,
}

impl Drop for TimeTrackerFileStore {
    fn drop(&mut self) {
        let duration_string = self.duration.to_string();
        match self.commit() {
            Ok(_) => println!("dropped TimeTrackerFileStore {}", duration_string),
            Err(err) => eprintln!("TimeTrackerFileStore:Drop: {}", err),
        }
    }
}

impl TimeTrackerFileStore {
    pub fn from_file(file: &Path) -> io::Result<TimeTrackerFileStore> {
        match OpenOptions::new().read(true).open(PathBuf::from(&file)) {
            Ok(mut f) => {
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => {
                        let duration = match ActiveDuration::from_str(buf.trim()) {
                            Ok(d) => d,
                            Err(_) => ActiveDuration::new(),
                        };
                        Ok(TimeTrackerFileStore {
                            duration,
                            file_path: OsString::from(file),
                        })
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err),
        }
    }

    pub fn resume(&mut self) -> io::Result<()> {
        println!("resume");
        self.duration.resume();
        self.commit()
    }

    pub fn stop(&mut self) -> io::Result<()> {
        println!("stop");
        self.duration.pause();
        self.commit()
    }

    pub fn duration(&self) -> &ActiveDuration {
        &self.duration
    }

    fn commit(&mut self) -> io::Result<()> {
        println!("commit");
        match OpenOptions::new().write(true).truncate(true).create(true).open(PathBuf::from(self.file_path.as_os_str()).as_path()) {
            Ok(mut f) => {
                match f.write(self.duration.to_string().as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        eprintln!("file::write: {}", err);
                        Err(err)
                    }
                }
            },
            Err(err) => Err(err),
        }
    }
}
