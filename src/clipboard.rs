use super::Error;
use std::io;

use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy(mut file: impl io::Read, strip_whitespace: bool) -> Result<Vec<u8>, Error> {
    let mut ctx = ClipboardContext::new()?;
    let mut contents = Vec::with_capacity(32);
    file.read_to_end(&mut contents)?;

    let mut contents = String::from_utf8(contents)?;
    if strip_whitespace {
        contents = contents.trim().to_string();
    }
    ctx.set_contents(contents.clone())?;
    Ok(contents.into())
}

pub fn paste(strip_whitespace: bool) -> Result<Vec<u8>, Error> {
    let mut ctx = ClipboardContext::new()?;
    let mut contents = ctx.get_contents()?;
    if strip_whitespace {
        contents = contents.trim().to_string();
    }
    Ok(contents.into_bytes())
}
