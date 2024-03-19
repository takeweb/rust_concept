fn ex_div_option(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

fn ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("dive by zero")
    } else {
        Ok(x / y)
    }
}

fn ex_print_option_if<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

fn ex_print_option_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn ex_print_result_match<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    ex_print_option_if(ex_div_option(10, 5));
    ex_print_option_if(ex_div_option(10, 0));

    ex_print_option_match(ex_div_option(10, 5));
    ex_print_option_match(ex_div_option(10, 0));

    ex_print_result_match(ex_div_result(10, 5));
    ex_print_result_match(ex_div_result(10, 0));
}
