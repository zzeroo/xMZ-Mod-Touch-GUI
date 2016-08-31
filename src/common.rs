use errors::*;


pub fn report_error(e: &Error) {
    error!("{}", e);

    for e in e.iter().skip(1) {
        info!("caused by: {}", e);
    }

    if show_backtrace() {
        info!("backtrace:");
        println!("");
        println!("{:?}", e.backtrace());
    } else {
    }

    fn show_backtrace() -> bool {
        use std::env;
        use std::ops::Deref;

        if env::var("RUST_BACKTRACE").as_ref().map(Deref::deref) == Ok("1") {
            return true;
        }

        for arg in env::args() {
            if arg == "-v" || arg == "--verbose" {
                return true;
            }
        }

        return false;
    }
}
