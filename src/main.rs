use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io::{self, prelude::*, BufWriter},
    path::Path,
    process,
};

/**
 * Runs the program.
 */
fn run(args: &[OsString]) -> Result<(), Box<dyn Error>> {
    match args.len() {
        0 | 1 => {
            // No output file name specified on command-line.  Use stdout.
            rays::run(&mut io::stdout().lock(), true)?;
        }

        2 => {
            // Get the output file name from the command-line.
            let filename = Path::new(&args[1]);

            let mut output = match File::create(filename) {
                Ok(output) => BufWriter::new(output),
                Err(x) => {
                    return Err(
                        format!("cannot open output file “{}”: {}", filename.display(), x).into(),
                    )
                }
            };

            rays::run(&mut output, true)?;

            if let Err(x) = output.flush() {
                return Err(format!("error writing to “{}”: {}", filename.display(), x).into());
            }
        }

        _ => {
            return Err("too many command-line arguments".into());
        }
    }

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
 * Usage: `rays [OUTPUT_FILE]`
 */
fn main() {
    let args: Vec<_> = env::args_os().collect();

    let progname = get_progname(&args);

    if let Err(x) = run(&args) {
        eprintln!("{}: {}", progname, x);
        process::exit(1);
    }
}
