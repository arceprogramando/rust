fn main() {
    let s1 = String::from("hola");

    let len = calcular_longitud(&s1);

    println!("La longitud de '{s1}' es {len}.");
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}