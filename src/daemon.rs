use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;

use super::util_temp;
use super::util_serial;

pub struct TempFanSpeedMapItem {
    temp: u32,
    fan_speed: u8,
    sleep_time: u64,
}

pub fn run(temp_file: PathBuf, serial_file: PathBuf) {
    let temp_fan_speed_map = vec![
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
    ];
    let mut prev_fan_speed: u8 = 255;
    loop {
        let temp = util_temp::get_temp(&temp_file);
        let temp = match temp {
            Ok(temp) => temp,
            Err(_) => {
                eprintln!("Error: Unable to get CPU temperature!");
                return ();
            }
        };
        println!("CPU temperature: {}", temp);
        let mut fan_speed: u8 = 0;
        let mut sleep_time: u64 = 0;
        for temp_fan_speed_item in &temp_fan_speed_map {
            if temp >= temp_fan_speed_item.temp {
                fan_speed = temp_fan_speed_item.fan_speed;
                sleep_time = temp_fan_speed_item.sleep_time;
                break;
            }
        }
        if prev_fan_speed != fan_speed {
            println!("Set fan speed to {}%", fan_speed);
            prev_fan_speed = fan_speed;
            util_serial::set_fan_speed(&serial_file, fan_speed);
        }
        if sleep_time < 1000 {
            sleep_time = 1000;
        }
        sleep(Duration::from_millis(sleep_time));
    }
}
