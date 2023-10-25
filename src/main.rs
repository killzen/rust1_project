mod first_module {
    pub fn print_chars() {
        for chr in 'a'..='z' {
            println!("{}", chr);
        }
        for chr in 'A'..='Z' {
            println!("{}", chr);
        }
    }
}

mod second_module {
    pub fn print_chars() {
        for chr in 'A'..='z' {
            println!("{}", chr);
        }
    }
}
fn main() {
    first_module::print_chars();
    second_module::print_chars();
}
