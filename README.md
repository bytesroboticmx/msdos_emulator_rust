# MS-DOS Emulator en Rust

Un emulador básico de MS-DOS escrito en Rust que proporciona un intérprete de comandos interactivo con soporte para comandos comunes del sistema operativo DOS.

## Características

- **Interfaz interactiva REPL** - Prompt que muestra el directorio actual
- **Comandos compatibles con DOS** - Sintaxis similar a MS-DOS
- **Operaciones reales con el sistema de archivos** - Navega y manipula archivos/directorios reales
- **Estructura modular** - Código organizado y fácil de extender
- **Soporte para rutas absolutas y relativas** - Al igual que en MS-DOS

## Comandos Disponibles

### Comandos Básicos
| Comando | Alias | Descripción |
|---------|-------|-------------|
| `echo [texto]` | - | Muestra un mensaje en pantalla. Sin argumentos muestra el estado de ECHO |
| `cls` | `clear` | Limpia la pantalla |
| `ver` | `version` | Muestra la versión del emulador |
| `help` | `?` | Muestra la lista de comandos disponibles |
| `rem [comentario]` | - | Agrega un comentario (no realiza ninguna acción) |
| `exit` | `quit` | Sale del emulador |

### Información del Sistema
| Comando | Descripción |
|---------|-------------|
| `date` | Muestra la fecha actual del sistema |
| `time` | Muestra la hora actual del sistema |
| `vol` | Muestra información del volumen |
| `mem` | Muestra información de la memoria |

### Sistema de Archivos
| Comando | Alias | Descripción |
|---------|-------|-------------|
| `dir [ruta]` | `ls` | Lista archivos y directorios |
| `cd [ruta]` | `chdir` | Cambia de directorio. Sin argumentos muestra el directorio actual |
| `pwd` | - | Muestra la ruta del directorio actual |
| `md <directorio>` | `mkdir` | Crea un nuevo directorio |
| `rd <directorio>` | `rmdir` | Elimina un directorio vacío |
| `type <archivo>` | `cat` | Muestra el contenido de un archivo |
| `del <archivo>` | `erase`, `rm` | Elimina un archivo |
| `copy <origen> <destino>` | - | Copia un archivo |
| `ren <origen> <nuevo_nombre>` | `rename` | Renombra un archivo o directorio |

## Uso

### Requisitos
- [Rust](https://www.rust-lang.org/) (versión 1.56 o superior)

### Instalación y Ejecución

1. Clonar o descargar el repositorio:
```bash
git clone <url-del-repositorio>
cd msdos_emulator_rust
```

2. Compilar y ejecutar:
```bash
cargo run
```

3. Usar el emulador:
```bash
MS-DOS Emulator v1.0 - Rust
Copyright (C) 2024 Dr. Aldo Gonzalez Vazquez

Escribe 'help' para ver la lista de comandos disponibles.
Escribe 'exit' para salir.

/ruta/actual> help
```

## Estructura del Proyecto

```
msdos_emulator_rust/
├── Cargo.toml          # Configuración del proyecto
├── README.md           # Documentación
└── src/
    ├── main.rs         # Punto de entrada y REPL
    ├── core/           # Núcleo del emulador
    │   ├── mod.rs
    │   └── emulator.rs # Estado del emulador (directorio actual)
    └── commands/       # Implementación de comandos
        ├── mod.rs      # Despachador de comandos
        ├── basic.rs    # Comandos básicos
        ├── system.rs   # Comandos de información del sistema
        └── filesystem.rs # Comandos del sistema de archivos
```

## Autor

Dr. Aldo Gonzalez Vazquez

## Licencia

Ver archivo [LICENSE](LICENSE) para más detalles.
