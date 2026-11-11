# Proyectos y sistemas desarrollables en Rust en el entorno actual

## Entorno actual (detectado)

- **Rust/Cargo**: `rustc 1.99.1` y `cargo 1.99.1`
- **SO/arquitectura**: Linux `aarch64`
- **Proyecto en curso**: emulador de MS-DOS en Rust (`/root/msdos_emulator_rust`, edition 2021, sin dependencias externas)
- **Capacidades prácticas**: compilación y ejecución de binarios CLI, pruebas con `cargo test`, uso de crates desde crates.io (si hay acceso a red), manipulación de archivos/procesos/red desde espacio de usuario

## Proyectos o sistemas que puedes desarrollar aquí

### 1. Extensiones del emulador MS-DOS actual
- **Intérprete por lotes**: soporte para scripts `.BAT` (`IF`, `GOTO`, variables de entorno, `CALL`).
- **Sistema de archivos virtual**: montar una imagen o un árbol simulado en memoria en lugar de operar sobre el FS real.
- **Más comandos DOS**: `ATTRIB`, `FIND`, `MORE`, `TREE`, `XCOPY`, `PATH`, manejo básico de redirecciones (`>`, `>>`, `|`).
- **Historial y edición de línea**: integrar `rustyline` para autocompletado e historial de comandos.
- **Pruebas automatizadas**: suite de tests unitarios/integración para el despachador y cada comando.

### 2. Herramientas de línea de comandos (CLI)
- **Utilidades de sistema**: buscador de archivos, renombrador masivo, monitor de uso de disco, limpiador de logs.
- **Gestores de tareas personales**: TODO/kanban en terminal con persistencia en JSON/SQLite (`serde`, `rusqlite`).
- **Parseadores y conversores**: CSV↔JSON↔TOML, validadores de formatos, generadores de reportes.
- **Automatización**: wrappers para flujos de build, despliegue o procesamiento por lotes.

### 3. Sistemas y servicios (backend/red)
- **Servidores HTTP/REST**: APIs con `axum`, `actix-web` o `warp` (servicios web, webhooks, microservicios).
- **Servicios TCP/UDP personalizados**: chat, eco, protocolos simples, proxies o balanceadores básicos.
- **Clientes de red**: scrapers, consumidores de APIs, sincronizadores de archivos.
- **Colas y workers**: procesamiento asíncrono con `tokio` y canales.

### 4. Emuladores, intérpretes y máquinas virtuales simples
- **Máquinas virtuales de pila/registro**: VM educativa con su propio bytecode y ensamblador.
- **Intérpretes de lenguajes**: calculadora, Lisp pequeño, BASIC/Logo simplificado, o un shell propio.
- **Emuladores retro (8-bit)**: Chip-8, CPU 8080/Z80 parcial o Game Boy (núcleo lógico en CLI; la parte gráfica requeriría dependencias/entorno gráfico adicional).
- **Simuladores**: CPU didáctica, autómatas, simuladores de procesos/planificación.

### 5. Datos, concurrencia y rendimiento
- **Motores de indexación/búsqueda**: índice invertido simple para archivos de texto.
- **Procesamiento paralelo**: pipelines con `rayon`, análisis de logs o cómputo numérico.
- **Bases de datos embebidas**: aplicaciones con `sqlite`, `sled` o almacenamiento clave-valor propio.
- **Compresión/serialización**: herramientas para empaquetar, deduplicar o transformar datos.

### 6. Seguridad y formatos binarios
- **Herramientas forenses básicas**: análisis de binarios, extracción de metadatos, hashing y verificación de integridad.
- **Parseadores de formatos**: ELF, PE, imágenes de disco (FAT) o archivos de configuración, usando crates como `nom`.
- **Criptografía aplicada**: cifrado de archivos, gestión de secretos o firmas (con crates auditadas).

## Consideraciones del entorno
- **Muy adecuado**: proyectos CLI, librerías, servidores, parsers, emulación lógica y automatización (todo lo anterior compila y corre bien en Linux/aarch64).
- **Condicionado**: aplicaciones gráficas/de escritorio o juegos con ventana (SDL, GUI nativa) requieren librerías del sistema y, en muchos casos, servidor gráfico; posible pero depende de instalar dependencias.
- **Punto de partida inmediato**: evolucionar el emulador existente en `src/` (ya compila y corre) es la opción con menor fricción para añadir funcionalidades y pruebas.
