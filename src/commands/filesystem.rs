use crate::core::MSDOSEmulator;
use std::fs;
use std::path::{Path, PathBuf};

fn resolve_path(base: &Path, args: &str) -> PathBuf {
    if args.is_empty() {
        return base.to_path_buf();
    }
    let path = Path::new(args);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

pub fn dir(current_dir: &PathBuf, args: &str) {
    let path = if args.is_empty() { current_dir.clone() } else { resolve_path(current_dir, args) };
    
    println!(" Directorio de {}", path.display());
    println!();
    
    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut count_files = 0;
            let mut count_dirs = 0;
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().ok();
                    let file_name = entry.file_name();
                    let name_str = file_name.to_string_lossy();
                    
                    if let Some(meta) = metadata {
                        if meta.is_dir() {
                            println!("    <DIR>         {}", name_str);
                            count_dirs += 1;
                        } else {
                            let size = meta.len();
                            println!("            {:>10} {}", size, name_str);
                            count_files += 1;
                        }
                    } else {
                        println!("                  {}", name_str);
                    }
                }
            }
            println!();
            println!("    {} archivo(s)      bytes", count_files);
            println!("    {} dir(s)", count_dirs);
        }
        Err(e) => {
            println!("No se encuentra el archivo o directorio.");
            println!("Error: {}", e);
        }
    }
}

pub fn cd(emulator: &mut MSDOSEmulator, args: &str) {
    if args.is_empty() {
        println!("{}", emulator.current_dir.display());
        return;
    }
    
    let new_path = resolve_path(&emulator.current_dir, args);
    
    match fs::metadata(&new_path) {
        Ok(meta) => {
            if meta.is_dir() {
                match fs::canonicalize(&new_path) {
                    Ok(canon) => {
                        emulator.current_dir = canon;
                    }
                    Err(_) => {
                        emulator.current_dir = new_path;
                    }
                }
            } else {
                println!("El directorio especificado no es válido.");
            }
        }
        Err(_) => {
            println!("El sistema no puede encontrar la ruta especificada.");
        }
    }
}

pub fn pwd(current_dir: &PathBuf) {
    println!("{}", current_dir.display());
}

pub fn mkdir(current_dir: &PathBuf, args: &str) {
    if args.is_empty() {
        println!("Falta el nombre del directorio.");
        return;
    }
    
    let new_dir = resolve_path(current_dir, args);
    match fs::create_dir(&new_dir) {
        Ok(_) => println!("Directorio creado correctamente."),
        Err(e) => println!("Error al crear el directorio: {}", e),
    }
}

pub fn rmdir(current_dir: &PathBuf, args: &str) {
    if args.is_empty() {
        println!("Falta el nombre del directorio.");
        return;
    }
    
    let dir = resolve_path(current_dir, args);
    match fs::remove_dir(&dir) {
        Ok(_) => println!("Directorio eliminado correctamente."),
        Err(e) => println!("Error al eliminar el directorio: {}", e),
    }
}

pub fn type_file(current_dir: &PathBuf, args: &str) {
    if args.is_empty() {
        println!("Falta el nombre del archivo.");
        return;
    }
    
    let file_path = resolve_path(current_dir, args);
    match fs::read_to_string(&file_path) {
        Ok(contents) => print!("{}", contents),
        Err(e) => println!("Error al leer el archivo: {}", e),
    }
}

pub fn del(current_dir: &PathBuf, args: &str) {
    if args.is_empty() {
        println!("Falta el nombre del archivo.");
        return;
    }
    
    let file_path = resolve_path(current_dir, args);
    match fs::remove_file(&file_path) {
        Ok(_) => println!("Archivo eliminado correctamente."),
        Err(e) => println!("Error al eliminar el archivo: {}", e),
    }
}

pub fn copy(current_dir: &PathBuf, args: &str) {
    let mut parts = args.splitn(2, char::is_whitespace);
    let src = match parts.next() {
        Some(s) if !s.is_empty() => s,
        _ => {
            println!("Sintaxis incorrecta. Uso: copy <origen> <destino>");
            return;
        }
    };
    let dst = match parts.next() {
        Some(s) if !s.is_empty() => s.trim(),
        _ => {
            println!("Sintaxis incorrecta. Uso: copy <origen> <destino>");
            return;
        }
    };
    
    let src_path = resolve_path(current_dir, src);
    let dst_path = resolve_path(current_dir, dst);
    
    match fs::copy(&src_path, &dst_path) {
        Ok(_) => println!("Archivo copiado correctamente."),
        Err(e) => println!("Error al copiar el archivo: {}", e),
    }
}

pub fn rename(current_dir: &PathBuf, args: &str) {
    let mut parts = args.splitn(2, char::is_whitespace);
    let src = match parts.next() {
        Some(s) if !s.is_empty() => s,
        _ => {
            println!("Sintaxis incorrecta. Uso: ren <origen> <nuevo_nombre>");
            return;
        }
    };
    let dst = match parts.next() {
        Some(s) if !s.is_empty() => s.trim(),
        _ => {
            println!("Sintaxis incorrecta. Uso: ren <origen> <nuevo_nombre>");
            return;
        }
    };
    
    let src_path = resolve_path(current_dir, src);
    let dst_path = resolve_path(current_dir, dst);
    
    match fs::rename(&src_path, &dst_path) {
        Ok(_) => println!("Archivo o directorio renombrado correctamente."),
        Err(e) => println!("Error al renombrar: {}", e),
    }
}
