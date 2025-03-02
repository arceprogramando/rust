fn main() {

    // Stack: &str, &[char; n], &[u8; n], &[T; n], &T (referencias)
    
    // Heap: String, Vec<u8>, Vec<T>, Box<T>, Rc<T>, Arc<T>, Mutex<T>, RefCell<T>, Cell<T>, Cow<T>, Pin<T>, Box<dyn Trait>, &dyn Trait, &mut dyn Trait, &T, &mut T (punteros)

    // Por defecto el mas recomendado es .to_string()
    // Es mas especifico por que le dices que quieres un String tambien el type annotation es mas especifico
    let mut mi_nombre:String= "Felipe".to_string();
    // El to_string referencia a el str asi que tiene todas las propiedades de un str
    println!("Mi nombre es {}",mi_nombre);

    mi_nombre.push_str(" Arce");
    println!("Mi nombre es {}",mi_nombre);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    println!("s is {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let mut s3 = s1 + &s2;
    s3.push('s');

    s3.push_str("Felipe");
    println!("s3 is {s3} ");

    //FORMAT

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}",s);

    let hello = String::from("Здравствуйте");

    let hellolen = hello.len();
    println!("{}",hellolen);

    for c in "Зд".chars() {
        println!("{c}");
    }
    
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
