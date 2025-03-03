// La función main no tiene parámetros y no devuelve nada

fn main() {
    let mut suma = sumar_dos_numeros(5, 6);
    println!("La suma de 5 + 6 es: {}", suma);

    suma = sumar_dos_numeros(10, 20);
    println!("La segunda de suma es de: {}", suma);

    println!("-------------------------");
    funcion_ejemplo();
}

// Se le llama firma de la función a la definición de la función que incluye el nombre de la función, los parámetros que recibe y el tipo de dato que devuelve.

// Argumento : Es el valor que se le pasa a la función
// Parámetro : Es el nombre que se le da a los argumentos dentro de la función
// Tipo de dato : Es el tipo de dato que se espera recibir o devolver
// Valor de retorno : Es el valor que devuelve la función

fn sumar_dos_numeros(a: i32, b: i32) -> i32 {
    let resultado = a + b;
    return resultado;
}

fn funcion_ejemplo() {
    println!("Función de ejemplo");
    println!("-------------------------");
    
    let productos = ["Laptop", "Celular", "Mouse", "Teclado"];
    let precios = [1000, 700, 100, 100];
    let stock = [23, 30, 3, 10];

    let colores_de_teclado = vec!["Negro", "Blanco", "Rosado", "Naranja"];
    let colores_de_mouse = vec!["Negro", "Gris", "Blanco", "Azul"];

    mostrar_productos(
        productos,
        precios,
        stock,
        &colores_de_teclado,
        &colores_de_mouse,
    );
}

fn mostrar_productos(
    productos: [&str; 4],
    precios: [i32; 4],
    stock: [i32; 4],
    colores_de_teclado: &Vec<&str>,
    colores_de_mouse: &Vec<&str>,
) {
    for indice in 0..4 {
        let lista_de_colores = match indice {
            0..=1 => vec![],
            2 => colores_de_mouse.clone(),
            3 => colores_de_teclado.clone(),
            _ => unreachable!(),
        };

        mostrar_articulo(
            productos[indice],
            precios[indice],
            stock[indice],
            lista_de_colores,
        );

        println!("-------------------------");
    }

    fn mostrar_articulo(producto: &str, precio: i32, stock: i32, colores: Vec<&str>) {
        println!("📦 Producto: {}", producto);
        println!("💲 Precio: ${}", precio);
        println!("📦 Stock disponible: {}", stock);

        if colores.is_empty() {
            println!("🎨 No tiene colores");
        } else {
            println!("🎨 Colores disponibles:");
            for color in colores {
                println!("  - {}", color);
            }
        }
    }
}
