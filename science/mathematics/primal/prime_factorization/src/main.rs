/// Простые числа
use primal::Sieve;

fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}

fn main() {
    let sieve = Sieve::new(10000);
    let n = 2610;
    println!(
        "Число {} имеет {} делителей",
        n,
        num_divisors(n, &sieve).unwrap(),
    );
}
