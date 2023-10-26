pub mod first_module {
    pub fn print_chars() {
        for chr in 'a'..='z' {
            println!("{}", chr);
        }
        for chr in 'A'..='Z' {
            println!("{}", chr);
        }
    }
}