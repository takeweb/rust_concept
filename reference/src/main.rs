fn myprint1<T: std::fmt::Display>(msg: T) {
    // リファレンスによってmsgを受け取る
    println!("{}", msg);
}

fn myprint2<T: std::fmt::Display>(msg: &T) {
    // リファレンスによってmsgを受け取る
    // println!("{}", *msg);
    println!("{}", msg);
}

fn main() {
    // let s = "Hello".to_string();
    let s = String::from("Hello");
    myprint1(s.clone());
    myprint1(s.clone());

    myprint2(&s);
    myprint2(&s);
}
