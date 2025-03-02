fn main() {

    // if simple
    let mi_condicion = true;

    if mi_condicion {
        println!("La condición es verdadera y va a entrar en el if");
    } else {
        println!("La condición es falsa y va a entrar en el else");
    }

    // match

    let mi_numero = 5;

    match mi_numero {
        1 => println!("El match : Es uno"),
        2 => println!("El match : Es dos"),
        3 => println!("El match : Es tres"),
        4 => println!("El match : Es cuatro"),
        5 => println!("El match : Es cinco"),
        6..10 => println!("El match : Es un numero entre 6 y 10"),
        _ => println!("No es ninguno de los anteriores") // _ es el default tambien se puede usar todo!()
    }

    let mi_tupla = ("Hola", 10);

    match mi_tupla {
        (_, 1..=11) => println!("El match : Es la tupla  1 correcta"),
        ("Hola",5..=12) => println!("El match : Es la tupla 2 correcta"),
        _ => println!("No es la tupla correcta")
        
    }

    let tengo_hambre = true;
    let hay_comida = false;

    match (tengo_hambre, hay_comida) {
        (true, true) => println!("El match : Tengo hambre y hay comida"),
        (true, false) => println!("El match : Tengo hambre pero no hay comida"),
        (false, true) => println!("El match : No tengo hambre pero hay comida"),
        (false, false) => println!("El match : No tengo hambre y no hay comida")
        
    }
}
