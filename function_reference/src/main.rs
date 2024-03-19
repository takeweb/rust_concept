// ライフタイム指定をした場合
fn pick<'a>(x: &'a [i32], end: usize) -> &'a [i32] {
    &x[..end]
}

// ライフタイム省略の場合
fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let p = pick(&v1, 2);
    for ss in p {
        println!("{}", ss);
    }

    let p = pick1(&v1, 2);
    for ss in p {
        println!("{}", ss);
    }
}
