use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MSDOSEmulator {
    pub current_dir: PathBuf,
}

impl MSDOSEmulator {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        MSDOSEmulator { current_dir }
    }
}
