use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write, stdin};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: String,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: String, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_completed(&self) -> bool {
        self.completed
    }
}

pub fn crear_id() -> String {
    let mut cadena = String::new();
    let mut rng = rand::thread_rng();
    for n in 1..=10 {
        if n % 2 == 0 {
            let numero: i32 = rng.gen_range(1..=10);
            if numero + n % 3 == 0 {
                let letra: u32 = rng.gen_range(65..=90);
                let ch = char::from_u32(letra).unwrap();
                cadena.push(ch);
            } else {
                let letra: u32 = rng.gen_range(97..=122);
                let ch = char::from_u32(letra).unwrap();
                cadena.push(ch);
            }
        } else {
            let numerotemp: i32 = rng.gen_range(1..=9);
            let num = numerotemp.to_string();
            cadena.push_str(&num);
        }
    }

    return cadena;
}

pub fn agregar_elemento(todo: Todo) {
    let ruta_archivo = "./data/Todos.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(ruta_archivo)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("No se pudo abrir el archivo: {}", e);
            return;
        }
    };
    let mut contenido = String::new();
    if let Err(e) = file.read_to_string(&mut contenido) {
        eprintln!("No se pudo leer el archivo: {}", e);
        return;
    }
    let mut todos: Vec<Todo> = match serde_json::from_str(&contenido) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };
    file.set_len(0).expect("No se pudo limpiar el archivo");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("No se pudo mover el cursor");
    todos.push(todo);
    let json = serde_json::to_string_pretty(&todos).expect("No se pudo convertir a JSON");
    file.write_all(json.as_bytes()).expect("Error");
    println!("Tarea agregado con exito");
}

pub fn completar_tarea(id: String) {
    let ruta_archivo = "./data/Todos.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(ruta_archivo)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("No se pudo abrir el archivo: {}", e);
            return;
        }
    };
    let mut contenido = String::new();
    if let Err(e) = file.read_to_string(&mut contenido) {
        eprintln!("No se pudo leer el archivo: {}", e);
        return;
    }
    let mut todos: Vec<Todo> = match serde_json::from_str(&contenido) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };
    todos.iter_mut().for_each(|todo| {
        if todo.id == id {
            todo.completed = !todo.completed;
        }
    });

    file.set_len(0).expect("No se pudo limpiar el archivo");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("No se pudo mover el cursor");

    let json = serde_json::to_string_pretty(&todos).expect("No se pudo convertir a JSON");
    file.write_all(json.as_bytes()).expect("Error");
    println!("Tarea swicheada con exito")
}

pub fn leer_linea(mut input: String) -> String {
    stdin()
        .read_line(&mut input)
        .expect("Error al leer la linea");
    return input;
}

pub fn leer_opcion() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("No es un número válido");
            0
        }
    }
}

pub fn mostrar_tareas() -> Vec<Todo> {
    let ruta_archivo = "./data/Todos.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(ruta_archivo)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("No se pudo abrir el archivo: {}", e);
            return Vec::new();
        }
    };
    let mut contenido = String::new();
    if let Err(e) = file.read_to_string(&mut contenido) {
        eprintln!("No se pudo leer el archivo: {}", e);
        return Vec::new();
    }
    let todos: Vec<Todo> = match serde_json::from_str(&contenido) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };
    return todos;
}
pub fn eliminar_tarea(id: String) {
    let ruta_archivo = "./data/Todos.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(ruta_archivo)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("No se pudo abrir el archivo: {}", e);
            return;
        }
    };
    let mut contenido = String::new();
    if let Err(e) = file.read_to_string(&mut contenido) {
        eprintln!("No se pudo leer el archivo: {}", e);
        return;
    }
    let mut todos: Vec<Todo> = match serde_json::from_str(&contenido) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };
    todos.retain(|todo| todo.id != id);

    file.set_len(0).expect("No se pudo limpiar el archivo");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("No se pudo mover el cursor");

    let json = serde_json::to_string_pretty(&todos).expect("No se pudo convertir a JSON");
    file.write_all(json.as_bytes()).expect("Error");
    println!("Tarea eliminada con exito");
}

pub fn modificar_tarea(id: String, titulo: String, descripcion: String) {
    let ruta_archivo = "./data/Todos.json";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(ruta_archivo)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("No se pudo abrir el archivo: {}", e);
            return;
        }
    };
    let mut contenido = String::new();
    if let Err(e) = file.read_to_string(&mut contenido) {
        eprintln!("No se pudo leer el archivo: {}", e);
        return;
    }
    let mut todos: Vec<Todo> = match serde_json::from_str(&contenido) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };
    todos.iter_mut().for_each(|todo| {
        if todo.id == id {
            todo.description = descripcion.to_string();
            todo.title = titulo.to_string();
        }
    });

    file.set_len(0).expect("No se pudo limpiar el archivo");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("No se pudo mover el cursor");

    let json = serde_json::to_string_pretty(&todos).expect("No se pudo convertir a JSON");
    file.write_all(json.as_bytes()).expect("Error");
    println!("Tarea modificada con exito")
}
