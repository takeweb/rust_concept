fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    println!("s= {}", s);
    println!("s's memory address= {:p}", &s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s= {}", s_ref);
    println!("s_ref's memory address= {:p}", s_ref);
}
