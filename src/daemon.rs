use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;

use super::util_gpio;
use super::util_serial;
use super::util_temp;

const SLEEP_TIME: u64 = 5000;

pub fn run(temp_file: PathBuf, serial_file: PathBuf, gpio: Option<u8>) {
    let mut lite_mode = false;
    let mut gpio_out_pin: Option<rppal::gpio::OutputPin> = match gpio {
        Some(gpio_pin) => match util_gpio::open_gpio_pin(gpio_pin) {
            Ok(pin) => {
                lite_mode = true;
                println!("Running daemon in Lite mode (GPIO pin: {})", gpio_pin);
                Some(pin)
            },
            Err(_) => {
                eprintln!("Error: Unable to open GPIO pin!");
                return ();
            },
        },
        None => {
            println!("Running daemon in Pro mode");
            None
        },
    };
    let temp_fan_speed_map = util_temp::get_default_temp_speed_map(lite_mode);
    let mut sleep_time: u64 = 0;
    let mut prev_fan_speed: u8 = 255;
    let mut fan_speed: u8 = 0;
    loop {
        let temp = util_temp::get_temp(&temp_file);
        let temp = match temp {
            Ok(temp) => temp,
            Err(_) => {
                eprintln!("Error: Unable to get CPU temperature!");
                return ();
            }
        };
        for temp_fan_speed_item in &temp_fan_speed_map {
            if temp >= temp_fan_speed_item.temp && fan_speed != temp_fan_speed_item.fan_speed {
                if temp_fan_speed_item.fan_speed > fan_speed || sleep_time == 0 {
                    fan_speed = temp_fan_speed_item.fan_speed;
                    sleep_time = temp_fan_speed_item.sleep_time;
                }
                break;
            }
        }
        if prev_fan_speed != fan_speed {
            println!("Set fan speed to {}% as CPU temp is {:.1}°C", fan_speed, temp as f32 / 1000.0);
            prev_fan_speed = fan_speed;
            match gpio_out_pin {
                Some(ref mut gpio_out_pin) => {
                    util_gpio::set_fan_speed(gpio_out_pin, fan_speed);
                },
                _ => {
                    util_serial::set_fan_speed(&serial_file, fan_speed);
                }
            }
        }
        sleep(Duration::from_millis(SLEEP_TIME));
        if sleep_time - SLEEP_TIME > 0 {
            sleep_time -= SLEEP_TIME;
        } else {
            sleep_time = 0;
        }
    }
}
