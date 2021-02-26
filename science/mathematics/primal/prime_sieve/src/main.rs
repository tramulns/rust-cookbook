/// Простые числа
use primal::Sieve;

fn main() {
    let sieve = Sieve::new(10000); // Решето Эратосфена

    let suspect = 5273;
    assert_eq!(true, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    assert_ne!(true, sieve.is_prime(not_a_prime));

    let n = 1000;
    match sieve.primes_from(0).nth(n - 1) {
        Some(number) => println!("{} простым числом является {}", n, number),
        None => println!("Не знаю {} простое число.", n),
    }
}
