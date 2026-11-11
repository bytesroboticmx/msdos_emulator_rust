pub mod basic;
pub mod filesystem;
pub mod system;

use crate::core::MSDOSEmulator;

pub fn interpret_command(emulator: &mut MSDOSEmulator, input: &str) {
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    // Split command and arguments
    let mut parts = input.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("").to_lowercase();
    let args = parts.next().unwrap_or("").trim().to_string();

    match command.as_str() {
        // Basic commands
        "echo" => basic::echo(&args),
        "exit" | "quit" => return, // Handled in main
        "help" | "?" => basic::help(),
        "cls" | "clear" => basic::cls(),
        "ver" | "version" => basic::ver(),
        "rem" => basic::rem(&args),
        
        // System info
        "date" => system::date(),
        "time" => system::time(),
        "vol" => system::vol(),
        "mem" => system::mem(),
        
        // Filesystem commands
        "dir" | "ls" => filesystem::dir(&emulator.current_dir, &args),
        "cd" | "chdir" => filesystem::cd(emulator, &args),
        "pwd" => filesystem::pwd(&emulator.current_dir),
        "md" | "mkdir" => filesystem::mkdir(&emulator.current_dir, &args),
        "rd" | "rmdir" => filesystem::rmdir(&emulator.current_dir, &args),
        "type" | "cat" => filesystem::type_file(&emulator.current_dir, &args),
        "del" | "erase" | "rm" => filesystem::del(&emulator.current_dir, &args),
        "copy" => filesystem::copy(&emulator.current_dir, &args),
        "ren" | "rename" => filesystem::rename(&emulator.current_dir, &args),
        
        _ => println!("Comando desconocido o error de sintaxis. Escribe 'help' para ver los comandos disponibles."),
    }
}
