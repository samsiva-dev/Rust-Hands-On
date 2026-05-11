// Traits Practice
use std::fmt; 

// Defining a trait
trait Summary {
    // No implementation, so ended with ";"
    // Required Method - implementor MUST provide this
    fn summarize_author(&self) -> String;

    // Default Implementation
    // implementor CAN OVERRIDE, but doesn't need to be
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

struct Book {
    title: String,
    author: String,
    genre: String,
}

impl Article {
    fn get_title(&self) -> String {
        self.title.clone()
    }
}

// Now Article should implement summarize_author, but not necessarilyy summarize()
impl Summary for Article {
    // Must implement, so IMPLEMENTed here
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // summarize() uses default implementation - no override needed
    // If we want to, we can
    fn summarize(&self) -> String {
        format!("{} is writing on title {}", self.summarize_author(), self.get_title())
    }

    // Now, if we call summarize() it prints (based on example in main())
    // "Siva is writing on title Rust Traits" 
    // If we content above override, it prints
    // "(Read more from Siva...)"

    /*
        Note: If you observe, I have defined get_title() in "impl Article{}" block not in
        "impl Summary for Article{}", because "impl Summary for Article{}" is sole responsibility
        is to override/implement the methods of Summary trait, no more no less.

        So to define Article related functions, we must use "impl Article{}". 

        And one more thing, we called "get_title()" in "impl Summary for Article{}". Is that allowed? YES
        Because at the end &self points to Article Object so it can access it.
    */
}

// ================= Trait Bounds on Generics ===========================
// Means, if a Generic type (T) is bound to a trait, types which implemented that trait are allowed to pass to that argument
// types which don't implement that trait cannot use these functions.
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Different syntax, but same as above
// fn notify<T>(item: &T) where T: Summary {
//     println!("{}", item.summarize());
// }

// fn notify(item: &impl Summary) {
//     println!("{}", item.summarize());
// }

// ======================================================================

// ===================== Supertraits ====================================
// Means to implement Animal, you MUST also implement Display first
trait Animal: fmt::Display {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn describe(&self) -> String {
        format!("{} goes '{}'", self.name(), self.sound())
    }
}

struct Dog {
    name: String
}

// struct Cat {
//     name: String 
// }

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Animal for Dog { // Allowed now, since super trait implemented
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Boww" }
}

// Throws: "`Cat` doesn't implement `std::fmt::Display`"
// impl Animal for Cat { // Not Allowed, since super trait not yet implemented
//     fn name(&self) -> &str { &self.name }
//     fn sound(&self) -> &str { "Meow" }
// }

fn main() {
    let article = Article {
        title: String::from("Rust Traits"),
        author: String::from("Siva"),
        content: String::from("An article on How Traits work in Rust")
    };

    let book = Book {
        title: String::from("Rust Guide"),
        author: String::from("Siva"),
        genre: String::from("Tech")
    };

    // println!("{}", article.summarize()); // prints "Siva is writing on title Rust Traits"

    notify(&article); // prints "Siva is writing on title Rust Traits"
    // notify(&book);  // throws error: "the trait bound `Book: Summary` is not satisfied"

    let dog = Dog {
        name: String::from("Rex")
    };
    println!("{}", dog.describe());
}