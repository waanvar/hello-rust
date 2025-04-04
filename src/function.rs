fn main() {
    hello_world();
    tell_height(170);

    let x: i32 = add(170, 46);

    println!("X is : {}", x);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn hello_world() {
    println!("Hello Rust");
}

fn tell_height(h: i32) {
    println!("Height : {}", h);
}
