use std::io;

fn main() {
    let fahrenheit = read_temperature("Ingrese la temperatura en Fahrenheit: ");
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{:.2} grados Fahrenheit son {:.2} grados Celsius", fahrenheit, celsius);
}

fn read_temperature(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la línea");
        
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Por favor, ingrese un número válido."),
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}