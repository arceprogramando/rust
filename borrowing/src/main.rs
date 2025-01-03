fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    println!("{r1} y {r2}");
    // variables r1 y r2 no se usaran más a partir de aquí

    let r3 = &mut s; // no hay problema
    println!("{r3}");
}