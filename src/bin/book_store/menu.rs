use std::io::{self, Write};

trait DisplayConsoleMenu {
    fn add_book();
    fn list_books();
    fn save_books_to_file();
    fn quit();
}

pub enum MenuOptions {
    AddBook,
    ListBooks,
    SaveToFile,
    Quit,
}

impl DisplayConsoleMenu for MenuOptions {
    fn add_book() {
        println!("1: Add a record.");
    }
    fn list_books() {
        println!("2: List all books.");
    }
    fn save_books_to_file() {
        println!("3: Save current books to database.");
    }
    fn quit() {
        println!("4: Quit");
    }
}

impl MenuOptions {
    pub fn show_menu() {
        MenuOptions::add_book();
        MenuOptions::list_books();
        MenuOptions::save_books_to_file();
        MenuOptions::quit();
    }
    pub fn get_console_option() -> Option<MenuOptions> {
        let mut choice = String::new();
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading from terminal");

        match choice.trim() {
            "1" => Some(MenuOptions::AddBook),
            "2" => Some(MenuOptions::ListBooks),
            "3" => Some(MenuOptions::SaveToFile),
            "4" => Some(MenuOptions::Quit),
            _ => None,
        }
    }
}
