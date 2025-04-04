fn main() {
    let _v: Vec<i32> = Vec::new();
    let _the_vec: Vec<i32> = vec![1, 2, 3];

    let mut _the_numbers_vec: Vec<i32> = Vec::new();
    _the_numbers_vec.push(4);
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    _the_numbers_vec.push(7);

    //let x: &i32 = &_the_numbers_vec[2];

    let third: Option<&i32> = _the_numbers_vec.get(2);

    match third {
        Some(x) => println!("Third value is {x}"),
        None => println!("Invalid"),
    }
}
