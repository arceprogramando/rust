fn main() {
    let s = String::from("hola");

    let len = first_word(&s);

    println!("La longitud de '{s}' es {len}.");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
