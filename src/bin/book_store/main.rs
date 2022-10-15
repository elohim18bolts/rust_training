use book_store::book::Book;
use std::fs::{File, OpenOptions};
use std::io::Read;
fn main() {
    let book = Book::new(
        "Lord of the rings",
        "This is an amzing fantasy story. Elf, Dwarfs and Humans.",
        30.0,
    );
    match book {
        Ok(book) => {
            println!(
                "title: {} \ndescription: {} \nprice: {}",
                book.get_title(),
                book.get_description(),
                book.get_price()
            );
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("book.db")
                .expect("Error openning file");
            book.save_to_file(&mut file);
        }
        Err(e) => println!("{e:?}"),
    }

    match Book::read_from_file("book.db") {
        Some(books) => {
            for book in books {
                println!(
                    "title: {}\ndescription: {}\nprice: ${}",
                    book.get_title(),
                    book.get_description(),
                    book.get_price()
                );
            }
        }
        None => println!("No books readed"),
    }
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
