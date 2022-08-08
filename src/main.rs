mod cli;
mod daemon;
mod util_serial;
mod util_temp;

fn main() {
    match cli::cli().get_matches().subcommand() {
        Some(("daemon", args)) => cli::run_daemon(args),
        Some(("get-temperature", args)) => cli::get_temperature(args),
        Some(("list-serials", _)) => cli::list_serials(),
        _ => unreachable!(),
    }
}
