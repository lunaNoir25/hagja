# Hagja (학자) - Rust's Scribe logger

A lightweight and robust logger system for rust.
Hagja provides clean outputs, logging to files, and coloured logs.

Copyright (c) 2025 Luna Moonlit Noir <lunaNoir.sk@gmail.com>
GNU General Public License v3.0

## Example

```rust
use hagja::*;
let id = "My rust project.";
let logger: Hagja = Hagja::new(id, LogLevel::Info, false, None);
set_default_logger(logger).expect("unable to set default logger");
info!("Hello, world!");
```

### Hagja::new()

`id: &'static str` = Unique identifer for your project.

`log_level: LogLevel` = Log level, use info for consumer ready use, debug or trace for development use.

`write_file: bool` = Write to output file or not.

`file: Option<Arc<Mutex<File>>>` = Set to `None` if `write_file` is set to false. Requires `std::fs::File` and `std::sync::{Arc, Mutex}`. Prior file creation needed first.
