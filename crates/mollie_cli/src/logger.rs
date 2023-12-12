use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init(cli_debug_mode: bool) {
    // When developing, we want a thorough logger with as much info as possible.
    if cfg!(debug_assertions) {
        build_local_logger(cli_debug_mode);
        return;
    }

    build_production_logger(cli_debug_mode);
}

fn get_level_filter(debug_mode: bool) -> LevelFilter {
    if debug_mode {
        return LevelFilter::Debug;
    }
    LevelFilter::Info
}

fn build_local_logger(cli_debug_mode: bool) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}:{} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .filter(None, get_level_filter(cli_debug_mode))
        .init();
}

fn build_production_logger(cli_debug_mode: bool) {
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, get_level_filter(cli_debug_mode))
        .init();
}
