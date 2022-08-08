use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub enum TempReadError {
    ParseError(std::num::ParseIntError),
    IoError(std::io::Error),
}

pub fn get_temp(file: &PathBuf) -> Result<u32, TempReadError> {
    match File::open(file) {
        Ok(mut file) => {
            let mut data = String::new();
            match file.read_to_string(&mut data) {
                Ok(_) => {
                    match data.trim().parse() {
                        Ok(t) => Ok(t),
                        Err(err) => Err(TempReadError::ParseError(err)),
                    }
                }
                Err(err) => Err(TempReadError::IoError(err)),
            }
        }
        Err(err) => Err(TempReadError::IoError(err)),
    }
}

pub fn get_temp_file(file: Option<&PathBuf>) -> PathBuf {
    match file {
        Some(file) => PathBuf::from(file),
        _ => PathBuf::from("/sys/devices/virtual/thermal/thermal_zone0/temp"),
    }
}
