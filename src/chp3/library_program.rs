/*
    Let us work on the menu of a library. Create a structure containing book information
    like accession number, name of author, book title and flag to know whether book is issued or not.
    Create a menu in which the following can be done.

    1 - Display book information
    2 - Add a new book
    3 - Display all the books in the library of a particular author
    4 - Display the number of books of a particular title
    5 - Display the total number of books in the library
    6 - Issue a book
    (If we issue a book, then its number gets decreased by 1 and if we add a book, its number gets increased by 1)
 */


pub mod library_program {
    use rand::random;
    use std::collections::HashMap;
    use std::io;
    use std::io::Write;

    struct Library {
        store: HashMap<i8, Book>
    }

    impl Library {
        fn display_all_books_of_an_author(&self, author_name: String ) {
            self.store.iter().for_each(|x| {
                let author = &x.1.author[..];
                if author == author_name {
                    println!("All books of author {} is  {:?}", author_name, &x);
                }
            })
        }

        fn display_book_information(&self, book_title: &str) {
            let res = self.store.iter().find(|x| {
                let (id, book) = x;
                &book.title[..] == book_title
            }).expect("Could not find book");

            println!("{:?}", res.1)
        }

        fn display_no_of_books_with_a_title(&self, book_title: &str) {
            let res = self.store.iter().filter(|x| {
                let (_, book) = x;

                &book.title[..] == book_title
            }).collect::<Vec<(&i8, &Book)>>().len();

            println!("Number of books with the {} title {}", book_title, res)
        }

        fn display_total_no_of_books(&self) -> usize {
            self.store.len()
        }

        fn add_book(&mut self, mut book: Book){
            let id: i8 = random();
            book.accession_number += 1;
            self.store.insert(id, book);
        }

        fn issue_book(&mut self, book_title: &str) {
            let mut get_book: &mut Book  = self.store.iter_mut().find(move |x| {
                let (_, book) = x;
                book.title == book_title
            }).unwrap().1;

            get_book.accession_number -= 1;
            get_book.is_issued = true;

        }

        fn new() -> Self {
            Library {
                store: HashMap::<i8, Book>::new()
            }
        }
    }

    #[derive(Debug)]
    struct Book{
        title: String,
        is_issued: bool,
        author: String,
        accession_number: i8,
    }

    impl Book {
        fn new(title: String, is_issued: bool, author: String, accession_number: i8) -> Book {
            Book{
                title,
                is_issued,
                author,
                accession_number
            }
        }
    }

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    pub fn book_main() {
        let mut lib = Library::new();
        let menu = concat!(
        "\n==Welcome to the Library store!==\n\n",
        "Please look at the menu and enter the corresponding number to execute the action\n",
        "1 - Display book information\n",
        "2 - Add a new book\n",
        "3 - Display all the books in the library of a particular author\n",
        "4 - Display the number of books of a particular title\n",
        "5 - Display the total number of books in the library\n",
        "6 - Issue a book\n"
        );

        loop {
            println!("{}", menu);

            io::stdout()
                .flush()
                .unwrap();

            let input = get_input();
            let choice = match input.parse::<u8>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a number");
                    continue;
                }
            };

            match choice {
                1 => {
                    println!("Enter the book title you want to look up");
                    // just collect input and
                    let input = get_input();
                    lib.display_book_information(&input);

                },
                2 => {
                    // ask user to enter the details of the book at each stage
                    // and try to construct a book struct instance
                    println!("Enter the details of the book you want to add");

                    println!("Enter the title of the book ");
                    let title = get_input();

                    println!("Enter the author's name");
                    let author_name = get_input();

                    println!("Is the book issued? ");
                    let is_issued: String = get_input();

                    println!("Enter the accession number? ");
                    let accession_number = get_input().parse::<i8>().unwrap();

                    let created_book = match is_issued.as_str() {
                        "yes" => Book::new(title, true, author_name, accession_number),
                        "no" => Book::new(title, false, author_name, accession_number),
                        _ => continue
                    };

                    lib.add_book(created_book);
                },
                3 => {
                    println!("Enter the particular author's book which you want display");
                    let author_name = get_input().parse().unwrap();
                    lib.display_all_books_of_an_author(author_name);
                },
                4 => {
                    println!("Enter the book title");
                    let book_title: String = get_input().parse().unwrap();
                    lib.display_no_of_books_with_a_title(&book_title);
                },
                5 => {
                    println!("total number of books is {}", lib.display_total_no_of_books());
                },
                6 => {
                    println!("Enter the book title to be issued");
                    let book_title: String = get_input().parse().unwrap();
                    lib.issue_book(&book_title);

                },
                _ => {}
            }
        }

    }

}

mod tests {
    use crate::chp3::library_program::library_program::book_main;

    #[test]
    pub fn test_book() {
        //TODO refactor to make testable
        assert_eq!((), book_main())
    }
}
