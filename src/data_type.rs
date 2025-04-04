fn main() {
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer : {}", x);
    println!("Unsigned Integer : {}", y);

    let pi: f64 = 3.14;
    println!("Value of pi : {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing ?  {}", is_snowing);

    let letter: char = 'a';
    println!("First letter is {}", letter);

    //Debuggable Format {:?}
    //Display Format {}

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array {:?}", numbers);

    let fruit: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruit array {:?}", fruit);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("This human {:?}", human);

    let mix: (String, i32, bool, [i32; 5]) = ("Alice".to_string(), 30, true, numbers);
    println!("This mix {:?}", mix);

    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
    println!("Number slice : {:?}", number_slices);

    let animal_slices: &[&str] = &["lion", "elephant", "chicken"];
    println!("Animal slice : {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"Harry".to_string(),
        &"GOT".to_string(),
        &"Dragon".to_string(),
    ];
    println!("Book slice : {:?}", book_slices);

    let mut ston_cole: String = String::from("Hey , ");
    ston_cole.push_str("Marry");
    println!("Stone cole say : {}", ston_cole);

    let string: String = String::from("Hello , World");
    let slice: &str = &string;
    println!("Slice value : {}", slice);
    println!("Slice 5 : {}", &slice[0..5]);
}
