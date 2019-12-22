use crate::ClipboardProvider;

use raw_window_handle::HasRawWindowHandle;
use std::error::Error;

pub fn new_clipboard<W: HasRawWindowHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(clipboard_windows::Clipboard::new()?))
    }

    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(clipboard_macos::Clipboard::new()?))
    }

    #[cfg(target_os = "ios")]
    {
        Ok(Box::new(clipboard_ios::Clipboard::new()?))
    }
}

#[cfg(target_os = "windows")]
impl ClipboardProvider for clipboard_windows::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }
}

#[cfg(target_os = "macos")]
impl ClipboardProvider for clipboard_macos::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }
}


#[cfg(target_os = "ios")]
mod clipboard_ios {
    use std::error::Error;
    pub struct Clipboard;
    impl Clipboard {
        pub fn new() -> Result<Clipboard, Box<dyn Error>> {
            Ok(Self)
        }
    }
}

#[cfg(target_os = "ios")]
impl ClipboardProvider for clipboard_ios::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!();
    }
}
