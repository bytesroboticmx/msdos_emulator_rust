pub fn echo(args: &str) {
    if args.is_empty() {
        println!("ECHO esta activado.");
    } else {
        println!("{}", args);
    }
}

pub fn help() {
    println!("Comandos disponibles:");
    println!();
    println!("  COMANDOS BASICOS:");
    println!("    echo [texto]    - Muestra mensajes en pantalla");
    println!("    cls, clear      - Limpia la pantalla");
    println!("    ver, version    - Muestra la version del MS-DOS");
    println!("    help, ?         - Muestra esta ayuda");
    println!("    rem [comentario]- Comentario (no hace nada)");
    println!("    exit, quit      - Sale del emulador");
    println!();
    println!("  INFORMACION DEL SISTEMA:");
    println!("    date            - Muestra/establece la fecha");
    println!("    time            - Muestra/establece la hora");
    println!("    vol             - Muestra la etiqueta del volumen");
    println!("    mem             - Muestra la memoria disponible");
    println!();
    println!("  SISTEMA DE ARCHIVOS:");
    println!("    dir, ls         - Lista archivos y directorios");
    println!("    cd, chdir [dir]- Cambia de directorio");
    println!("    pwd             - Muestra directorio actual");
    println!("    md, mkdir <dir>- Crea un directorio");
    println!("    rd, rmdir <dir>- Elimina un directorio");
    println!("    type, cat <arc>- Muestra contenido de archivo");
    println!("    del, erase, rm <arc> - Elimina un archivo");
    println!("    copy <origen> <dest> - Copia un archivo");
    println!("    ren, rename <orig> <nuevo> - Renombra archivo/directorio");
    println!();
}

pub fn cls() {
    // Clear screen using ANSI escape code
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

pub fn ver() {
    println!("MS-DOS Emulator (Rust) Version 1.1");
    println!("Copyright (C) 2026 Dr. Aldo Gonzalez Vazquez");
    println!("Rust MS-DOS Emulation Project");
}

pub fn rem(_args: &str) {
    // REM is a comment, does nothing
}
