fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'h');
    another_function();
    let sum = sumtwonumbers(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);

    let x = five();

    println!("The value of x is: {x}");


}

fn another_function() {
    println!("Another function.");
}

//add two values ​​of type i32
//i32 is a 32-bit data type, meaning it allows
//integer values ​​from -2,147,483,648 to 2,147,483,647, does not allow decimals

fn sumtwonumbers(a: i32, b: i32) -> i32 {
    a + b
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
