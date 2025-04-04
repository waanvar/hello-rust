fn main() {
    let mut r: i32 = 5;
    let x: &mut i32 = &mut r;

    *x += 1;

    println!("X is {}", x);
    println!("R is {}", r);
}
