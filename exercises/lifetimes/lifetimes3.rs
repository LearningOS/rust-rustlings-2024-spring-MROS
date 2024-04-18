// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// XXX: 結構體裡放多個生命週期標記有什麼用途？ 'a, 'b 必須活得比 Book 都還要長嗎？
struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
