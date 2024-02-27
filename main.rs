//Programa en Rust que emula el sistema MS-DOS al ejecutar un comando basico en consola 'echo' y tambien 'exit'.
//Autor: Dr. Aldo Gonzalez Vazquez
//Version 1.0
//Fecha: 26/02/2024
use std::io::{self, Write};

struct MSDOSEmulator {
    // Aquí puedes agregar el estado del emulador, como registros, memoria, etc.
}

impl MSDOSEmulator {
    fn new() -> Self {
        MSDOSEmulator {
            // Inicializa el estado aquí
        }
    }

    fn interpret_command(&mut self, command: &str) {
        match command {
            "echo" => self.echo_command(),
            _ => println!("Comando desconocido"),
        }
    }

    fn echo_command(&self) {
        println!("ECHO es un comando para mostrar mensajes en pantalla.");
    }
}

fn main() {
    let mut emulator = MSDOSEmulator::new();
    let mut input = String::new();

    println!("Emulador de MSDOS simple. Escribe 'echo' para probar.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        if command == "exit" {
            break;
        }

        emulator.interpret_command(command);
    }
}
