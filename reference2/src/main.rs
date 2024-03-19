fn myprint<T: std::fmt::Display>(msg: &T) {
    // リファレンスによってmsgを受け取る
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let s_ref = &s;

    // イミュータブルなリファレンス
    myprint(s_ref);
    myprint(s_ref);
}
