enum Publication {
    Books(Book),
    Magazines(Magazine),
}

struct Book {
    id: u32,
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    id: u32,
    title: String,
    issue: u32,
    topic: String,
}

fn print_publications(publications: Vec<Publication>) {
    for p in publications {
        match p {
            Publication::Books(ref b) => {
                println!("=> ID: {}, Book Title: {}, Author: {}, Page Count: {}\n", b.id, b.title, b.author, b.page_count);
            }
            Publication::Magazines(ref m) => {
                println!("=> ID: {}, Magazine Title: {}, Issue {}, Topic: {}\n", m.id, m.title, m.issue, m.topic)
            }
        }
    }
}

fn main() {
    let book = Book {
        id: 1,
        title: "Rust Programming".to_string(),
        author: "John Doe".to_string(),
        page_count: 325,
    };

    let magazine = Magazine {
        id: 2,
        title: "Yoga Monthly".to_string(),
        issue: 202,
        topic: "Fitness".to_string(),
    };

    let publications = vec![
        Publication::Books(book),
        Publication::Magazines(magazine)
    ];

    print!("\nMy Publications: \n\n");
    print_publications(publications);
}