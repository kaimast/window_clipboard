use crate::ClipboardProvider;
use std::error::Error;

use raw_window_handle::HasRawWindowHandle;

pub struct NullClipboard {}

pub fn connect<W: HasRawWindowHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    Ok(Box::new(NullClipboard{}))
}

impl ClipboardProvider for NullClipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(""))
    }

    fn write(&mut self, _contents: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
