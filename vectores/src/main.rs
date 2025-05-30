fn main() {

    // Memoia RAM
    // STACK | HEAP
    // Stack -> i32, f64, bool, char, u8, u16, u32, u64, u128, isize, usize, references, etc.

    // HEAP -> Vec<T>, String, Box<T>, Rc<T>, RefCell<T>, etc.


    // Se crea en el heap el vector pero ademas se guarda la dirección en memoria del heap en el stack
    
    let mut v: Vec<u8> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let third: &u8 = &v[2];
    println!("The third element is {third}");

    let six: Option<&u8> = v.get(1);
    match six {
        Some(six) => println!("The six element is {six}"),
        None => println!("There is no six element."),
    }

    println!("FOR:");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;

        println!("{i}");
    }
}
