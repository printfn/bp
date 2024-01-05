use std::{
    env, error, ffi, fmt, fs,
    io::{self, Write},
    process,
};

mod clipboard;

pub type Error = Box<dyn error::Error + Send + Sync + 'static>;

fn stdin_tty() -> bool {
    io::IsTerminal::is_terminal(&io::stdin())
}

fn stdout_tty() -> bool {
    io::IsTerminal::is_terminal(&io::stdout())
}

#[derive(Debug)]
struct UnknownArgumentError(ffi::OsString);
impl fmt::Display for UnknownArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown argument `{}`", self.0.to_string_lossy())
    }
}
impl error::Error for UnknownArgumentError {}

fn inner_main() -> Result<(), Error> {
    let mut input_file = None;
    let mut force_stdout = false;
    let mut output_to_stderr = false;
    let mut strip_whitespace = false;
    for arg in env::args_os().skip(1) {
        if arg == "-h" || arg == "--help" {
            print_usage(io::stdout())?;
            return Ok(());
        } else if arg == "-O" || arg == "--stdout" {
            force_stdout = true;
        } else if arg == "-E" || arg == "--stderr" {
            output_to_stderr = true;
        } else if arg == "-s" || arg == "--strip" {
            strip_whitespace = true;
        } else if arg
            .as_os_str()
            .to_str()
            .map_or(false, |s| s.starts_with('-'))
        {
            return Err(UnknownArgumentError(arg))?;
        } else if input_file.is_none() {
            input_file = Some(arg);
        } else {
            return Err(UnknownArgumentError(arg))?;
        }
    }

    let mut tty_paste = false;

    let contents = if let Some(input_file) = input_file {
        let file = fs::File::open(input_file)?;
        clipboard::copy(file, strip_whitespace)?
    } else if stdin_tty() {
        if !output_to_stderr {
            tty_paste = true;
        }
        clipboard::paste(strip_whitespace)?
    } else {
        clipboard::copy(io::stdin(), strip_whitespace)?
    };

    if force_stdout || tty_paste || !stdout_tty() {
        io::stdout().write_all(&contents)?;
        if tty_paste
            && !contents.ends_with(&[b'\n'])
            && !contents.is_empty()
            && stdout_tty()
            && !strip_whitespace
        {
            println!();
        }
    }

    if output_to_stderr {
        io::stderr().write_all(&contents)?;
    }

    Ok(())
}

fn print_usage(mut f: impl io::Write) -> io::Result<()> {
    write!(
        f,
        "Usage: bp [FLAGS] [file]

Flags:
    -h  --help      show usage information
    -O  --stdout    print clipboard contents to stdout
    -E  --stderr    print clipboard contents to stderr
    -s  --strip     strip whitespace from the content\n"
    )?;
    Ok(())
}

fn main() {
    match inner_main() {
        Ok(()) => (),
        Err(e) => {
            eprint!("Error: {}", e);
            while let Some(e) = e.source() {
                eprint!(": {}", e);
            }
            eprintln!();
            process::exit(1);
        }
    }
}
