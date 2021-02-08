/// Вывод отладочного сообщения в консоль
/// RUST_LOG=debug cargo run
use log::debug;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
