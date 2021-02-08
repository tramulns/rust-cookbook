/// Компиляция и статическая компоновка с идущей в комплекте библиотекой на C++

extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", multiply(5, 7));
    }
}
