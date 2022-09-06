use rppal::gpio::{Gpio, Error, OutputPin};

pub fn open_gpio_pin(pin: u8) -> Result<OutputPin, Error> {
    match Gpio::new() {
        Ok(gpio_inst) => {
            match gpio_inst.get(pin) {
                Ok(gpio_pin) => {
                    Ok(gpio_pin.into_output())
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

pub fn set_fan_speed(gpio_pin: &mut OutputPin, fan_speed: u8) {
    match fan_speed {
        100 => {
            gpio_pin.set_high();
        },
        0 => {
            gpio_pin.set_low();
        },
        _ => {
            match gpio_pin.set_pwm_frequency(200.0, fan_speed as f64/100.0) {
                Ok(_) => {},
                Err(_) => {
                    eprintln!("Error: Unable to set PWM with GPIO!")
                }
            }
        }
    }
}
