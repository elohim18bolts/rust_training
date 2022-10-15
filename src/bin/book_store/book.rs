use std::fs::File;
use std::io::{Read, Write};
const BOOK_TITLE_SIZE: usize = 80;
const BOOK_DESC_SIZE: usize = 200;
const PRICE_SIZE: usize = std::mem::size_of::<f32>();
const CHUNK_SIZE: usize = BOOK_DESC_SIZE + BOOK_TITLE_SIZE + PRICE_SIZE;
#[derive(Debug)]
pub struct Book {
    title: [u8; BOOK_TITLE_SIZE],
    description: [u8; BOOK_DESC_SIZE],
    price: f32,
}
impl Book {
    pub fn new(title: &str, description: &str, price: f32) -> Result<Book, String> {
        if title.len() > BOOK_TITLE_SIZE {
            return Err(String::from("title is too long"));
        } else if description.len() > BOOK_DESC_SIZE {
            return Err(String::from("description is too long"));
        }
        let mut t: [u8; BOOK_TITLE_SIZE] = [0; BOOK_TITLE_SIZE];
        let mut desc: [u8; BOOK_DESC_SIZE] = [0; BOOK_DESC_SIZE];
        let mut count = 0;
        for b in title.bytes() {
            t[count] = b;
            count += 1;
        }
        count = 0;
        for b in description.bytes() {
            desc[count] = b;
            count += 1;
        }
        Ok(Self {
            title: t,
            description: desc,
            price,
        })
    }
    pub fn get_title(&self) -> String {
        let mut title = vec![];
        for byte in self.title.iter() {
            if byte == &0 {
                break;
            }
            title.push(byte.clone());
        }
        String::from_utf8(title).expect("Error reading title")
    }
    pub fn get_description(&self) -> String {
        let mut desc = vec![];
        for byte in self.description.iter() {
            if byte == &0 {
                break;
            }
            desc.push(byte.clone());
        }
        String::from_utf8(desc).expect("Error reading title")
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn save_to_file(&self, file: &mut File) -> bool {
        let mut chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        let mut count = 0;
        for i in 0..BOOK_TITLE_SIZE {
            chunk[i] = self.title[i];
            count += 1;
        }
        for (i, j) in (BOOK_TITLE_SIZE..BOOK_DESC_SIZE).zip(0..BOOK_DESC_SIZE) {
            chunk[i] = self.description[j];
        }
        count = BOOK_DESC_SIZE;
        for byte in self.price.to_be_bytes().iter() {
            chunk[count] = *byte;
            count += 1;
        }

        match file.write(&chunk) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn read_from_file(file_path: &str) -> Option<Vec<Self>> {
        let mut books: Vec<Book> = vec![];
        let mut chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        match File::open(file_path) {
            Ok(mut file) => loop {
                match file.read(&mut chunk) {
                    Ok(size) => {
                        if size == 0 {
                            return Some(books);
                        }

                        let mut title: [u8; BOOK_TITLE_SIZE] = [0; BOOK_TITLE_SIZE];
                        for i in 0..BOOK_TITLE_SIZE {
                            title[i] = chunk[i];
                        }
                        let mut description: [u8; BOOK_DESC_SIZE] = [0; BOOK_DESC_SIZE];
                        for (i, j) in (0..BOOK_DESC_SIZE).zip(BOOK_TITLE_SIZE..BOOK_DESC_SIZE) {
                            description[i] = chunk[j];
                        }
                        let mut price: [u8; PRICE_SIZE] = [0; PRICE_SIZE];
                        for (i, j) in (0..PRICE_SIZE).zip(BOOK_DESC_SIZE..CHUNK_SIZE) {
                            price[i] = chunk[j];
                        }
                        books.push(Self {
                            title,
                            description,
                            price: f32::from_be_bytes(price),
                        });
                    }
                    Err(_) => return None,
                }
            },
            Err(_) => None,
        }
    }
}
