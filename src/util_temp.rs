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

pub struct TempFanSpeedMapItem {
    pub temp: u32,
    pub fan_speed: u8,
    pub sleep_time: u64,
}

pub fn get_default_temp_speed_map() -> Vec<TempFanSpeedMapItem> {
    vec![
        TempFanSpeedMapItem {
            temp: 52000,
            fan_speed: 100,
            sleep_time: 180000,
        },
        TempFanSpeedMapItem {
            temp: 45000,
            fan_speed: 75,
            sleep_time: 120000,
        },
        TempFanSpeedMapItem {
            temp: 43000,
            fan_speed: 50,
            sleep_time: 90000,
        },
        TempFanSpeedMapItem {
            temp: 40000,
            fan_speed: 25,
            sleep_time: 60000,
        },
        TempFanSpeedMapItem {
            temp: 0,
            fan_speed: 0,
            sleep_time: 30000,
        },
    ]
}
