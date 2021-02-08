/// Сложение комплексных чисел
use num::complex::Complex;

fn main() {
    let complex_num1 = Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
