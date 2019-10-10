
pub fn print_on_error<T, E>(result: Result<T, E>) 
    where E: std::error::Error 
{
    if let Err(err) = result {
        print_error_chain(&err);
    }
}

fn print_error_chain(e: &dyn std::error::Error) {
    print_error(e, None, None);
    let mut cause = e.source();
    while let Some(e) = cause {
        print_error(e, Some("Caused by: "), None);
        cause = e.source();
    }
}

fn print_error(e: &dyn std::error::Error, err_prefix: Option<&str>, 
    #[allow(unused_variables)] backtrace_prefix: Option<&str>) {

    let err_prefix = err_prefix.unwrap_or("Error: ");
    eprintln!("{}{}", err_prefix, e);
        
    #[cfg(feature = "backtrace")]
    {
        let bt_prefix = backtrace_prefix.unwrap_or("Backtrace:\n");
        if let Some(b) = e.backtrace() {
            eprintln!("{}{}", bt_prefix, b);
        }
    }
}