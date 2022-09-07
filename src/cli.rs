use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::path::PathBuf;

use super::daemon;
use super::util_serial;
use super::util_temp;

pub fn cli() -> Command<'static> {
    Command::new("deskpi-fan-control")
        .about("Fan control driver for DeskPi Pro/Lite")
        .author("Victor Zhestkov <vzhestkov@gmail.com>")
        .version("0.1.0")
        .infer_subcommands(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .propagate_version(true)
        .arg(
            Arg::new("version")
                .long("version")
                .short('V')
                .help("Print version information")
                .global(true)
        )
        .subcommand(
            clap::command!("daemon")
                .about("Run the daemon to control the fan")
                .display_order(1)
                .arg(
                    Arg::new("serial_file")
                        .long("serial-file")
                        .short('s')
                        .help("The path to a serial device file")
                        .display_order(3)
                        .takes_value(true)
                        .value_name("SERIAL_FILE")
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("temp_file")
                        .long("temperature-file")
                        .short('t')
                        .help("The path to a sensor file to read data from")
                        .display_order(4)
                        .takes_value(true)
                        .value_name("TEMP_FILE")
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("lite")
                        .long("lite")
                        .short('l')
                        .help("Run in Lite mode")
                        .display_order(1)
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("gpio")
                        .long("gpio-pin")
                        .short('g')
                        .help("Use GPIO pin")
                        .display_order(2)
                        .takes_value(true)
                        .value_name("PIN")
                        .value_parser(value_parser!(u8))
                )
        )
        .subcommand(
            clap::command!("get-temperature")
                .about("Get the current temperature of the CPU from the sensor")
                .display_order(2)
                .arg(
                    Arg::new("temp_file")
                        .long("temperature-file")
                        .short('t')
                        .help("The path to a sensor file to read data from")
                        .takes_value(true)
                        .value_name("TEMP_FILE")
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("raw")
                        .long("raw")
                        .short('r')
                        .help("Return raw temperature value")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            clap::command!("list-serials")
                .about("List all serial ports")
                .display_order(3)
        )
}

pub fn get_temperature(args: &ArgMatches) {
    let raw = match args.get_one::<bool>("raw") {
        Some(raw) => *raw,
        _ => false,
    };
    let file = util_temp::get_temp_file(args.get_one::<PathBuf>("temp_file"));
    let temp = util_temp::get_temp(&file);
    match temp {
        Ok(temp) => {
            if raw {
                println!("{}", temp);
            } else {
                println!("CPU temperature: {:7.3}°C", temp as f32 / 1000.0);
            }
        }
        Err(_) => eprintln!("Error: Unable to read temperature!"),
    }
}

pub fn list_serials() {
    match util_serial::list_serials() {
        Ok(ports) => {
            for p in ports {
                println!("{}", p.port_name);
            }
        }
        Err(_) => {
            eprintln!("Error: Unable to get serial ports list!");
        }
    }
}

pub fn run_daemon(args: &ArgMatches) {
    let temp_file = util_temp::get_temp_file(args.get_one::<PathBuf>("temp_file"));
    let serial_file = util_serial::get_serial_file(args.get_one::<PathBuf>("serial_file"));
    let gpio: Option<u8> = match args.get_one::<u8>("gpio") {
        Some(gpio_pin) => Some(*gpio_pin),
        _ => match args.get_one::<bool>("lite") {
            Some(true) => Some(14),
            _ => None,
        },
    };
    match serial_file {
        Ok(serial_file) => daemon::run(temp_file, serial_file, gpio),
        Err(_) => {
            eprintln!("Error: Unable to open serial file!");
        }
    }
}
