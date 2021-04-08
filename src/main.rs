use std::{env, error::Error, ffi::OsString, path::Path, process};

/**
 * Runs the program.
 */
fn run(_args: &[OsString]) -> Result<(), Box<dyn Error>> {
    // FIXME: Implement.
    Ok(())
}

/**
 * Returns the program name from the command-line.
 */
#[must_use]
fn get_progname(args: &[OsString]) -> &str {
    if let Some(progname) = args.get(0) {
        if let Some(progname) = Path::new(progname).file_name() {
            if let Some(progname) = progname.to_str() {
                if !progname.is_empty() {
                    return progname;
                }
            }
        }
    }
    "rays"
}

/**
 * Entry point.
 *
 * Usage: `rays`
 */
fn main() {
    let args: Vec<_> = env::args_os().collect();

    let progname = get_progname(&args);

    if let Err(x) = run(&args) {
        eprintln!("{}: {}", progname, x);
        process::exit(1);
    }
}
