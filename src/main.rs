mod garden;
mod first_module;
use first_module::first_module::print_chars;
use garden::second_module;


//use garden::first_module;
fn main() {
    print_chars();  //第一层
    second_module::print_chars(); //第二层

}