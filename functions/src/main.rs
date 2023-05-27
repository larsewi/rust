fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
}

fn another_function(x: i32, label: char) {
    println!("The value is {x}{label}");
}
