use serialport::{Result, SerialPortInfo};
use std::path::PathBuf;

pub fn list_serials() -> Result<Vec<SerialPortInfo>> {
    serialport::available_ports()
}

pub fn get_serial_file(file: Option<&PathBuf>) -> Result<PathBuf> {
    match file {
        Some(file) => {
            let file = PathBuf::from(file);
            if file.exists() {
                Ok(file)
            } else {
                Err(serialport::Error::new(
                    serialport::ErrorKind::NoDevice,
                    "File does not exist",
                ))
            }
        }
        _ => {
            let try_serials = vec![
                PathBuf::from("/dev/ttyDPIFAN0"),
                PathBuf::from("/dev/ttyUSB0"),
            ];
            for try_serial in try_serials {
                if try_serial.exists() {
                    return Ok(try_serial);
                }
            }
            Ok(PathBuf::from("/dev/null"))
        }
    }
}

pub fn set_fan_speed(file: &PathBuf, fan_speed: u8) {
    match serialport::new(file.to_string_lossy(), 9600).open() {
        Ok(mut serial_port) => {
            println!("Sending PWM_{:03} to {}", fan_speed, file.to_string_lossy());
            match serial_port.write(format!("PWM_{:03}\n", fan_speed).as_bytes()) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
        Err(err) => eprintln!("Error: on opening port {}: {}", file.to_string_lossy(), err),
    }
}
