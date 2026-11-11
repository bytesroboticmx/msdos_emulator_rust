mod commands;
mod core;

use core::MSDOSEmulator;
use std::io::{self, Write};

fn main() {
    let mut emulator = MSDOSEmulator::new();
    let mut input = String::new();

    println!("MS-DOS Emulator v1.1 - Rust");
    println!("Copyright (C) 2026 Dr. Aldo Gonzalez Vazquez");
    println!();
    println!("Escribe 'help' para ver la lista de comandos disponibles.");
    println!("Escribe 'exit' para salir.");
    println!();

    loop {
        print!("{}> ", emulator.current_dir.display());
        if let Err(_) = io::stdout().flush() {
            break;
        }
        input.clear();
        if let Err(_) = io::stdin().read_line(&mut input) {
            break;
        }
        let command_str = input.trim_end_matches(&['\r', '\n'][..]).to_string();
        
        // Check for exit
        let trimmed = command_str.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or("").to_lowercase();
        if cmd == "exit" || cmd == "quit" {
            println!("Saliendo del emulador MS-DOS...");
            break;
        }
        
        commands::interpret_command(&mut emulator, trimmed);
    }
}
