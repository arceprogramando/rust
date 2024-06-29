fn main() {
    // addition
    let _sum = 5 + 10;
    println!("{}", _sum);
    // subtraction
    let _difference = 95.5 - 4.3;
    println!("{}", _difference);
    // multiplication
    let _product = 4 * 30;
    println!("{}", _product);
    // division
    let _quotient = 56.7 / 32.2;
    println!("{}", _quotient);
    let _truncated = -5 / 3; // Results in -1
    println!("{}", _truncated);
    // remainder
    let _remainder = 43 % 5;
    println!("{}", _remainder);
    // boolean
    let t = true;
    println!("{}", t);
    // boolean with explicit type annotation
    let f: bool = false;
    println!("{}", f);
    // character
    let c = 'z';
    println!("{}", c);
    // character with explicit type annotation
    let z: char = 'ℤ';
    println!("{}", z);
    // unicode character 
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    // unicode string with explicit type annotation
    let chinesehello: &str = "你好";
    println!("{}", chinesehello);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    // destructuring tuple
    let first_tuple = tup.0;
    println!("{}", first_tuple);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // array , the arrays is used when you want to store a fixed number of elements  , if you need a collection that can grow or shrink in size, you’ll want to use a vector instead because an array is not flexible.

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("My birthday is: {}", months[9]);

    // array with explicit type annotation
    let array_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array_explicit);
    println!("The value of a[2] is: {}", array_explicit[2]);
}
