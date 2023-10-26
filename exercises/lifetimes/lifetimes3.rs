// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT_DONE

struct Book <'L>{
    author: &'L str,
    title: &'L str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    let name = "Jill Smith";
    let title = "Fish Flying";
    let book = Book { author: &name, title: &title };

    let book = Book { author: "Jill Smith", title: "Fish Flying" };

    println!("{} by {}", book.title, book.author);
}
