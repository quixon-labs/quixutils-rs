use std::io::Result;

macro_rules! write_error_chain_checked {
    ($e: expr, $to: expr) => {
        if let Err(e) = crate::errors::write_error_chain($e, $to) {
            panic!("failed to write: {:?}", e);
        }
    };
}

pub fn eprint_on_error<T, E>(result: std::result::Result<T, E>)
where
    E: std::error::Error,
{
    if let Err(err) = result {
        eprint_error_chain(&err);
    }
}

pub fn eprint_error_chain(e: &dyn std::error::Error) {
    let mut out = std::io::stderr();
    write_error_chain_checked!(e, &mut out);
}

pub fn log_error_chain(e: &dyn std::error::Error) {
    let mut err_bytes = Vec::with_capacity(16);
    write_error_chain_checked!(e, &mut err_bytes);
    let log_message = if err_bytes.ends_with(&['\n' as u8]) {
        &err_bytes[0..(err_bytes.len() - 1)]
    } else {
        &err_bytes[..]
    };
    log::error!("{}", String::from_utf8_lossy(log_message));
}

pub fn write_error_chain<W: std::io::Write>(e: &dyn std::error::Error, to: &mut W) -> Result<()> {
    write_error_chain_with_opts(e, None, Some("Caused By: "), to)
}

pub fn write_error_chain_with_opts<W: std::io::Write>(
    e: &dyn std::error::Error,
    err_prefix: Option<&str>,
    err_cause_prefix: Option<&str>,
    to: &mut W,
) -> Result<()> {
    write_error(e, err_prefix, to)?;
    let mut cause = e.source();
    while let Some(e) = cause {
        write_error(e, err_cause_prefix, to)?;
        cause = e.source();
    }
    Ok(())
}

pub fn write_error<W: std::io::Write>(
    e: &dyn std::error::Error,
    err_prefix: Option<&str>,
    to: &mut W,
) -> Result<()> {
    let err_prefix = err_prefix.unwrap_or("Error: ");
    writeln!(to, "{}{}", err_prefix, e)?;

    #[cfg(feature = "unstable")]
    {
        if let Some(b) = e.backtrace() {
            writeln!(to, "{}", b)?;
        }
    }

    Ok(())
}