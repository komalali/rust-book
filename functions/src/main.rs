fn main() {
    println!("Hello, world!");

    let x= plus_one(five());
    another_function(x, 8);
}

fn five(_result {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
