use std::fs::File;
use std::io::{Write, BufReader, BufRead};

// followed module 3 file operations examples

#[derive(Debug)]
struct Book {
    title: String, //index 0
    author: String, //index 1
    year: u16, //index 2
}

//all books need to be saved in a file
fn save_books(books: &Vec<Book>, filename: &str) { 
    // book needs to be in separate line w/ fields separated by commas (no whitespace)
    //create file to save books in
    let mut file = File::create(filename).unwrap(); //unwrap used for success/error of creating file

    //write books to file by iterating each book
    for book in books {
        //write!(file,"title: {}, author: {}, year: {}\n", book.title, book.author, book.year).unwrap(); // book has details mentioned
        write!(file,"{},{},{}\n", book.title, book.author, book.year).unwrap(); // book details only seperated by comma
    }
}

//loads books from a file
fn load_books(filename: &str) -> Vec<Book> { 
    // read books from file and return Vec<Book>
    //create new vector to return
    let mut books: Vec<Book> = Vec::new();

    //open file
    let file = File::open(filename).unwrap();

    //read file
    let read_books = BufReader::new(file);

    //go through each book info (one book per line in file)
    for book in read_books.lines() {
        let book = book.unwrap();

        //split string to sections to return to vector (similar to past assignment)
        let info: Vec<&str> = book.split(',').collect();

        //title and author needs to be strings (doesn't need unwrap())
        let title = info[0].to_string();
        let author = info[1].to_string();

         //convert year to u16 as it was converted to a string
            // use .parse()
        let year = info[2].parse::<u16>().unwrap();
        
        //push to new vector as a struct
        books.push(Book {title, author, year});
    }
    //return book vector 
        // title, author, year separated when it printed in main
    books
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