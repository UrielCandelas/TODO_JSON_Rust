mod todo {
    pub mod todo;
}
use todo::todo::Todo;
use todo::todo::{
    agregar_elemento, completar_tarea, crear_id, eliminar_tarea, leer_linea, leer_opcion,
    modificar_tarea, mostrar_tareas,
};

fn imprimir_menu() {
    println!("Ingrese una opcion:");
    println!("1.- Crear tarea");
    println!("2.- Completar/Descompletar tarea");
    println!("3.- Mostrar tareas");
    println!("4.- Eliminar tarea");
    println!("5.- Modificar tarea");
    println!("6.- Finalizar programa");
}
fn main() {
    let mut bandera = true;
    while bandera {
        imprimir_menu();
        let mut opc = leer_opcion();
        while opc < 1 || opc > 6 {
            imprimir_menu();
            opc = leer_opcion();
        }
        match opc {
            1 => {
                let id = crear_id();
                let mut titulo = String::new();
                let mut descripcion = String::new();

                println!("Ingrese el titulo de la tarea");
                titulo = leer_linea(titulo).trim_end().to_string();

                println!("Ingrese la descripcion de la tarea");
                descripcion = leer_linea(descripcion).trim_end().to_string();

                let todo = Todo::new(id, titulo, descripcion);
                agregar_elemento(todo);
            }
            2 => {
                let mut id = String::new();
                println!("Ingrese el id de la tarea a completar");
                id = leer_linea(id);
                completar_tarea(id.trim_end().to_string());
            }
            3 => {
                let todos = mostrar_tareas();
                if todos.len() > 0 {
                    for i in todos {
                        println!("Id: {}", i.get_id());
                        println!("Titulo: {}", i.get_title());
                        println!("Descripcion: {}", i.get_description());
                        println!(
                            "Completada: {}",
                            if i.get_completed() { "Si" } else { "No" }
                        );
                        println!("-----------------");
                    }
                }
            }
            4 => {
                let mut id = String::new();
                println!("Ingrese el id de la tarea a eliminar");
                id = leer_linea(id);
                eliminar_tarea(id.trim_end().to_string());
            }
            5 => {
                let mut id = String::new();
                let mut titulo = String::new();
                let mut descripcion = String::new();

                println!("Ingrese el id de la tarea a eliminar");
                id = leer_linea(id).trim_end().to_string();

                println!("Ingrese el titulo de la tarea");
                titulo = leer_linea(titulo).trim_end().to_string();

                println!("Ingrese la descripcion de la tarea");
                descripcion = leer_linea(descripcion).trim_end().to_string();

                modificar_tarea(id, titulo, descripcion);
            }
            6 => bandera = false,
            _ => println!("gg"),
        }
    }
}
