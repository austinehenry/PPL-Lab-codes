#[derive(Clone, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    // Constructor for a new book
    fn new(title: &str, author: &str, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            is_issued: false,
        }
    }

    // Function to issue a book (transfer ownership)
    fn issue_book(self) -> Book {
        // Transfer ownership here
        let mut book = self;
        book.is_issued = true; // Mark the book as issued
        book
    }

    // Function to clone the book to maintain a backup
    fn clone_for_backup(&self) -> Book {
        self.clone() // Clone the book
    }

    // Function to print details of the book
    fn print_details(&self) {
        println!("Book Details - Title: {}, Author: {}, ISBN: {}", self.title, self.author, self.isbn);
    }
}

fn main() {
    // Create a new book
    let library_book = Book::new("The Great Gatsby", "F. Scott Fitzgerald", "1234567890");

    // Print book details before issue
    library_book.print_details();

    // Backup the book before issuing (clone it)
    let backup_book = library_book.clone_for_backup();

    println!("Library book before issue: {:?}", library_book);

    // Issue the book to a borrower
    let issued_book = library_book.issue_book();

    // The library_book is now moved, so we cannot access it anymore
    println!("Issued book: {:?}", issued_book);

    // Backup still accessible
    println!("Backup of the library book: {:?}", backup_book);
}
