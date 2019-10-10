pub fn print_on_error<T, E>(result: Result<T, E>)
where
    E: std::error::Error,
{
    if let Err(err) = result {
        print_error_chain(&err);
    }
}

pub fn print_error_chain(e: &dyn std::error::Error) {
    print_error_chain_with_prefix(e, None, Some("Caused By: "));
}

pub fn print_error_chain_with_prefix(
    e: &dyn std::error::Error,
    err_prefix: Option<&str>,
    err_cause_prefix: Option<&str>,
) {
    print_error(e, err_prefix);
    let mut cause = e.source();
    while let Some(e) = cause {
        print_error(e, err_cause_prefix);
        cause = e.source();
    }
}

#[allow(unused_variables)]
pub fn print_error(e: &dyn std::error::Error, err_prefix: Option<&str>) {
    let err_prefix = err_prefix.unwrap_or("Error: ");
    eprintln!("{}{}", err_prefix, e);

    #[cfg(feature = "unstable")]
    {
        if let Some(b) = e.backtrace() {
            eprintln!("{}", b);
        }
    }
}
