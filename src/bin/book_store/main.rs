use book_store::book::Book;
use book_store::menu::MenuOptions;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn show_options() {
    MenuOptions::show_menu();
    println!("Select any options from above:");
}
fn read_from_terminal() -> String {
    let mut buff = String::new();
    print!("> ");
    std::io::stdout()
        .flush()
        .expect("Error: Cannot flush terminal.");
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Error: Error reading book property from terminal.");
    buff.trim().to_owned()
}
fn add_book(books: &mut Vec<Book>) {
    let mut title = String::new();
    let mut description = String::new();
    let mut price: f32 = f32::default();
    println!("Insert book title: ");
    title = read_from_terminal();
    println!("Insert book description: ");
    description = read_from_terminal();
    loop {
        let mut price_str = String::new();
        println!("Insert book price: ");
        price_str = read_from_terminal();
        match price_str.parse::<f32>() {
            Ok(p) => {
                price = p;
                break;
            }
            Err(_) => println!("Wrong price format. Please insert a number."),
        }
    }
    match Book::new(&title, &description, price) {
        Ok(book) => books.push(book),
        Err(e) => println!("{e:?}"),
    }
}
fn list_books(books: &Vec<Book>) {
    println!("Title: ");
    for book in books {
        println!("{title}", title = book.get_title());
    }
}
fn save_books_to_file(books: &Vec<Book>, file_path: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Error in opening the file");
    for book in books {
        if !book.save_to_file(&mut file) {
            println!("Error in saving {}", book.get_title());
        }
    }
}
fn init() {
    //Show menu options.
    let mut books: Vec<Book> = vec![];
    loop {
        show_options();
        match MenuOptions::get_console_option() {
            Some(option) => match option {
                MenuOptions::AddBook => add_book(&mut books),
                MenuOptions::ListBooks => list_books(&books),
                MenuOptions::SaveToFile => save_books_to_file(&books, "books.db"),
                MenuOptions::Quit => break,
            },
            None => {
                println!("Wrong option. Please select the correct option.");
                show_options();
            }
        }
    }
}
fn main() {
    init();
}

#[cfg(test)]
mod test {
    use book_store::book::Book;
    #[test]
    fn testing_book_creation() {
        let book = Book::new("hello", "hello", 23.0);
        assert_eq!(true, book.is_ok());
        assert_eq!(String::from("hello"), book.unwrap().get_title());
    }

    #[test]
    fn test_book_creation_error() {
        let long_title = "this is a reallllllllly loooooong tititititittile";
        let book = Book::new(long_title, "hello", 23.0);
        assert_eq!(
            true,
            book.is_err(),
            "This should be an error because title is too long"
        );
    }
}
