use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};


#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    priority: String,
    completed: bool,
}

// Function to read user input from the console
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn añadir(conn: &mut PooledConn) {
    println!("\n== Agregar Nueva Tarea ==");

    let description = read_input("Descripción de la tarea: ");
    if description.is_empty() {
        println!("La descripción no puede estar vacía.");
        return;
    }

    let priority = loop {
        let p = read_input("Prioridad (Baja, Media, Alta): ").to_lowercase();
        match p.as_str() {
            "baja" | "media" | "alta" => break p.to_string(),
            _ => println!("Prioridad no válida. Por favor, elija entre Baja, Media o Alta."),
        }
    };

    conn.exec_drop(
        "INSERT INTO tasks (description, priority, completed) VALUES (?, ?, ?)",
        (description, priority, false),
    ).expect("Error al insertar la tarea.");

    println!("✅ Tarea agregada exitosamente.");
}


fn ver(conn: &mut PooledConn) {
    println!("\n== Lista de Tareas ==");

    let tasks = conn.query_map(
        "SELECT id, description, priority, completed FROM tasks ORDER BY created_at DESC",
        |(id, description, priority, completed): (u32, String, String, bool)| {
            Task { id, description, priority, completed }
        },
    ).unwrap();

    if tasks.is_empty() {
        println!("No hay tareas registradas.");
        return;
    }

    println!("{:<5} {:<50} {:<10} {:<12}", "ID", "Descripción", "Prioridad", "Completada");
    println!("{}", "-".repeat(80));

    for task in tasks {
        let status = if task.completed { "Sí" } else { "No" };
        let priority_capitalized = task.priority.chars().next().unwrap().to_uppercase().to_string() + &task.priority[1..];
        println!(
            "{:<5} {:<50} {:<10} {:<12}",
            task.id, task.description, priority_capitalized, status
        );
    }
}


fn editar(conn: &mut PooledConn) {
    let id: u32 = match read_input("\nIngrese el ID de la tarea a editar: ").parse() {
        Ok(i) => i,
        Err(_) => {
            println!("ID inválido.");
            return;
        }
    };

    if conn.exec_first::<u32, _, _>("SELECT id FROM tasks WHERE id = ?", (id,)).unwrap().is_none() {
        println!("No existe una tarea con el ID {}.", id);
        return;
    }

    println!("\n¿Qué desea editar de la tarea ID {}?", id);
    println!("1. Descripción");
    println!("2. Prioridad");
    println!("3. Marcar como completada/pendiente");
    println!("0. Volver");

    match read_input("Opción: ").as_str() {
        "1" => {
            let new_description = read_input("Nueva descripción: ");
            if new_description.is_empty() {
                println!("La descripción no puede estar vacía.");
                return;
            }
            conn.exec_drop("UPDATE tasks SET description = ? WHERE id = ?", (new_description, id)).unwrap();
            println!("Descripción actualizada.");
        }
        "2" => {
            let new_priority = loop {
                let p = read_input("Nueva prioridad (Baja, Media, Alta): ").to_lowercase();
                match p.as_str() {
                    "baja" | "media" | "alta" => break p,
                    _ => println!("Prioridad no válida."),
                }
            };
            conn.exec_drop("UPDATE tasks SET priority = ? WHERE id = ?", (new_priority, id)).unwrap();
            println!("Prioridad actualizada.");
        }
        "3" => {
            let completed_str = read_input("¿La tarea está completada? (s/n): ").to_lowercase();
            let is_completed = matches!(completed_str.as_str(), "s" | "si" | "sí");
            conn.exec_drop("UPDATE tasks SET completed = ? WHERE id = ?", (is_completed, id)).unwrap();
            println!("Estado de la tarea actualizado.");
        }
        "0" => return,
        _ => println!("Opción no válida."),
    }
}


fn borrar(conn: &mut PooledConn) {
    let id: u32 = match read_input("ID de la tarea a eliminar: ").parse() {
        Ok(i) => i,
        Err(_) => {
            println!("ID inválido.");
            return;
        }
    };

    let result = conn.exec_drop("DELETE FROM tasks WHERE id = ?", (id,));
    match result {
        Ok(_) => println!("🗑️ Tarea eliminada exitosamente."),
        Err(e) => println!("Error al eliminar la tarea: {:?}", e),
    }
}


fn main() {
    let url = "mysql://roottos2:Locopa2.@localhost:3306/task_manager";
    let pool = Pool::new(url).expect("No se pudo conectar a la base de datos.");
    let mut conn = pool.get_conn().unwrap();

    loop {
        println!("\n====== MENÚ GESTOR DE TAREAS ======");
        println!("1. Agregar tarea");
        println!("2. Ver tareas");
        println!("3. Editar tarea");
        println!("4. Eliminar tarea");
        println!("5. Salir");

        match read_input("Opción: ").as_str() {
            "1" => añadir(&mut conn),
            "2" => ver(&mut conn),
            "3" => editar(&mut conn),
            "4" => borrar(&mut conn),
            "5" => {
                println!("👋 ¡Adiós!");
                break;
            }
            _ => println!("❌ Opción inválida."),
        }
    }
}