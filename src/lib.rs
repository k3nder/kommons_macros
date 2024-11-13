#[macro_use]
#[macro_export]
macro_rules! read_file {
    ($path:expr) => {{
        use std::fs::File;
        use std::io::{self, Read};

        // Intentamos abrir el archivo
        let mut file = match File::open($path) {
            Ok(file) => file,
            Err(e) => panic!("Can't open file {}: {}", $path, e),
        };

        let mut contents = String::new();
        // Intentamos leer el contenido del archivo
        if let Err(e) = file.read_to_string(&mut contents) {
            panic!("Can't read file {}: {}", $path, e);
        }

        contents
    }};
}

#[macro_use]
#[macro_export]
macro_rules! write_file {
    ($path:expr, $content:expr) => {{
        use std::fs::File;
        use std::io::{self, Write};

        // Intentamos abrir el archivo en modo de escritura (sobrescribiendo el contenido)
        let mut file = match File::create($path) {
            Ok(file) => file,
            Err(e) => panic!("Can't read/write file {}: {}", $path, e),
        };

        // Intentamos escribir el contenido
        if let Err(e) = file.write_all($content.as_bytes()) {
            panic!("Can't write file {}: {}", $path, e);
        }
    }};
}

#[macro_export]
macro_rules! append_file {
    ($path:expr, $content:expr) => {{
        use std::fs::OpenOptions;
        use std::io::{self, Write};

        // Intentamos abrir el archivo en modo de añadir (sin sobrescribir el contenido)
        let mut file = match OpenOptions::new().append(true).open($path) {
            Ok(file) => file,
            Err(e) => panic!("Can't write file {} in append mode: {}", $path, e),
        };

        // Intentamos escribir el contenido al final del archivo
        if let Err(e) = file.write_all($content.as_bytes()) {
            panic!("Can't write file in append mode {}: {}", $path, e);
        }
    }};
}

#[macro_use]
#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::{self, Write};

        // Creamos un nuevo String para almacenar la entrada del usuario
        let mut input = String::new();

        // Intentamos leer desde la entrada estándar (stdin)
        io::stdin().read_line(&mut input).expect("Can't read input");

        // Retornamos el input sin los caracteres de nueva línea (\n)
        input.trim().to_string()
    }};
    ($prompt:expr) => {{
        use std::io::{self, Write};

        // Mostramos el mensaje (prompt) al usuario
        print!("{}", $prompt);
        io::stdout().flush().expect("Can't write buffer");

        // Creamos un nuevo String para almacenar la entrada del usuario
        let mut input = String::new();

        // Intentamos leer desde la entrada estándar (stdin)
        io::stdin().read_line(&mut input).expect("Can't read input");

        // Retornamos el input sin los caracteres de nueva línea (\n)
        input.trim().to_string()
    }};
}



