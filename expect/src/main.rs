use std::fs::File;

fn main() {
    File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}