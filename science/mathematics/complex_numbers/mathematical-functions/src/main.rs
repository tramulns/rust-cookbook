/// Математических функции с комплексными числами
use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
