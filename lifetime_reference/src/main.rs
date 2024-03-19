fn return_hello1() -> String {
    "Hello".to_string()
}

fn return_hello2() -> &'static str {
    static RETURN_VALUE: &str = "Hello";
    RETURN_VALUE
}

fn hello3(s: &mut String) {
    s.push('3')
}

fn hello4(s: &mut String) -> &String {
    s.push('4');
    s
}

fn main() {
    let s1 = return_hello1();
    println!("{}", s1);

    let s2 = return_hello2();
    println!("{}", s2);

    let mut s3 = "abc".to_string();
    hello3(&mut s3);
    println!("{}", s3);

    let mut s4 = "def".to_string();
    hello4(&mut s4);
    println!("{}", s4);
}
