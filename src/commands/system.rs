use std::process::Command;

pub fn date() {
    // Try to get system date
    let output = Command::new("date").arg("+%d/%m/%Y").output();
    match output {
        Ok(out) => {
            let date_str = String::from_utf8_lossy(&out.stdout);
            print!("Fecha actual: {}", date_str);
            println!("Introduce nueva fecha (dd-mm-aa): ");
        }
        Err(_) => {
            println!("Fecha actual: No disponible");
        }
    }
}

pub fn time() {
    let output = Command::new("date").arg("+%H:%M:%S").output();
    match output {
        Ok(out) => {
            let time_str = String::from_utf8_lossy(&out.stdout);
            print!("Hora actual: {}", time_str);
            println!("Introduce nueva hora: ");
        }
        Err(_) => {
            println!("Hora actual: No disponible");
        }
    }
}

pub fn vol() {
    println!("Etiqueta de volumen no disponible en este emulador.");
    println!("Número de serie del volumen: XXXX-XXXX");
}

pub fn mem() {
    println!("Memoria:");
    println!("  Memoria convencional:     640 KB");
    println!("  Memoria superior:           0 KB");
    println!("  Memoria extendida:     65536 KB");
    println!("  Memoria total:         66176 KB");
    println!();
    println!("  Memoria libre:         60000 KB");
}
