/// Проверка количество логических ядер процессора

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
