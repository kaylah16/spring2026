use std::fs::File;
use std::io::{Write, BufReader, BufRead};

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

//all books need to be saved in a file
fn save_books(books: &Vec<Book>, filename: &str) { 
    // Hint: Use File::create() and write!() macro
    // book needs to be in separate line w/ fields separated by commas

    //create file to save books in
    let mut file = File::create(filename).unwrap(); //unwrap used for success/error of creating file

    //write books to file
    for book in books {
        write!(file,"title: {}, author: {}, year: {}\n", book.title, book.author, book.year).unwrap();
    }
    
}

//loads books from a file
fn load_books(filename: &str) -> Vec<Book> { 
    // Hint: Use File::open() and BufReader
    // read books from file and return Vec<Book>

    //create new vector to return
    let mut books: Vec<Book> = Vec::new();
    //open file
    let mut file = File::open(filename).unwrap();

    //read file
    let read_books = BufReader::new(file);
    for book in read_books.lines() {
        
    }

}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}