/// Проверка того, что тангенс равен синусу, разделённому на косинус

fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
