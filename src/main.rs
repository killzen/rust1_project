mod garden;
mod first_module;
use first_module::first_module::print_chars;
use garden::second_module;


//use garden::first_module;
fn main() {
    print_chars();
    second_module::print_chars(); // 调用 second_module.rs 中的函数
}