use core::f64;

enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn divide(numerator: f64, denominator: f64) -> MyOption<f64> {
    if denominator == 0.0 {
        MyOption::None
    } else {
        MyOption::Some(numerator / denominator)
    }
}

fn divide_check(numerator: f64, denominator: f64) -> MyResult<f64, String> {
    if denominator == 0.0 {
        MyResult::Err("Cannot div by zero".to_string())
    } else {
        MyResult::Ok(numerator / denominator)
    }
}

fn main() {
    enum IpAddress {
        V4(String),
        V6(String),
    }
    /*
       fn route(ip_kind: IpAddress) {}
       route(IpAddress::V4);
       route(IpAddress::V6);
       let four = IpAddress::V4;
    */

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from(":::1"));

    fn print_ip(ip: IpAddress) {
        match ip {
            IpAddress::V4(addr) => println!("IPv4 address: {}", addr),
            IpAddress::V6(addr) => println!("IPv6 address: {}", addr),
        }
    }

    print_ip(home);
    print_ip(loopback);

    let result: MyResult<f64, String> = divide_check(10.0, 0.0);
    /*
    match result {
        MyOption::Some(x) => println!("Result is : {} ", x),
        MyOption::None => println!("Cannot divide by Zero!"),
    }
     */
    match result {
        MyResult::Ok(x) => println!("Result is : {} ", x),
        MyResult::Err(y) => println!("Error : {}", y),
    }
}
