use log::LevelFilter;
use std::io::prelude::*;
use chrono::prelude::*;
use std::str::FromStr;
use log::Record;
use env_logger::fmt::Formatter;
use std::io;

pub fn init(verbosity: usize) {
    use env_logger::*;
    use std::env;

    const LOG_LEVEL_ENV: &'static str = "LOG_LEVEL";
    const LOG_LOCALTIME_ENV: &'static str = "LOG_LOCALTIME";

    let mut env = Env::new();
    let mut has_opts = false;
    // Use default RUST_LOG env
    let env_level = env::var(DEFAULT_FILTER_ENV);
    if env_level.is_ok() {
        has_opts = true;
    }

    // Use app specific LOG_LEVEL env that overrides RUST_LOG
    // Useful to debug app only when executed through other tools like `cargo run`.
    let env_level = env::var(LOG_LEVEL_ENV);
    if env_level.is_ok() {
        env = env.filter(LOG_LEVEL_ENV);
        has_opts = true;
    }

    let mut builder = Builder::from_env(env);
    if !has_opts {
        // Default log level
        builder.filter_level(level_filter_from_verbosity(verbosity));
    }

    let ts_local = match env::var(LOG_LOCALTIME_ENV) {
        Ok(v) => i32::from_str(&v).unwrap_or_default() > 0,
        Err(_) => false,
    };

    builder.format(get_formatter(ts_local));
    builder.init();
}

fn get_formatter(ts_local: bool) -> impl Fn(&mut Formatter, &Record) -> io::Result<()> + Sync + Send + 'static {
    move |buf: &mut Formatter, record: &Record| {
        use env_logger::fmt::Color;

        let l = record.level();
        let lvl_style = buf.default_level_style(l);
        let mut ts_style = buf.style();
        let ts_style = ts_style.set_color(Color::Rgb(140, 140, 140));
        let mut path_style = buf.style();
        let path_style = path_style.set_color(Color::Cyan);

        let ts = if ts_local {
            Local::now().to_rfc3339_opts(SecondsFormat::Millis, false)
        } else {
            Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
        };
        writeln!(buf, "{} {} [{}] {}",
                 lvl_style.value(l),
                 ts_style.value(ts),
                 path_style.value(record.target()),
                 record.args())
    }
}

fn level_filter_from_verbosity(verbosity: usize) -> LevelFilter {
    use log::LevelFilter::*;
    match verbosity {
        0 => Warn,
        1 => Info,
        2 => Debug,
        3 => Trace,
        _ => Trace,
    }
}
