/// Компиляция библиотеки на C с использованием особых директив

extern "C" {
    fn print_app_info();
}

fn main() {
    unsafe {
        print_app_info();
    }
}
