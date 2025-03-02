fn main() {
    println!("Bucle For");

    let mi_lista_de_usuarios = vec![
        ("Felipe", "Arce", 25),
        ("Juan", "Perez", 30),
        ("Maria", "Gonzalez", 40),
    ];

    for (index  , (nombre, apellido, edad)) in mi_lista_de_usuarios.into_iter().enumerate() {
        println!("Elemento: {}", index + 1);
        println!("Nombre: {}", nombre);
        println!("Apellido: {}", apellido);
        println!("Edad: {}", edad);
        println!("-------------------");
    }
}
