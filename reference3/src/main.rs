fn myprint<T: std::fmt::Display>(msg: &T) {
    // リファレンスによってmsgを受け取る
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let s_ref1 = &s;
    let s_ref2 = &s;

    // イミュータブルなリファレンス
    myprint(s_ref1);
    myprint(s_ref1);

    // 同じ値に対する別のイミュータブルなリファレンス
    myprint(s_ref2);
    myprint(s_ref2);
}
