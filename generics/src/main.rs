fn LARGEST(LIST: &[I32]) -> I32 {
    LET MUT LARGEST = LIST[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest1<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

fn main3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 1, y: 4.0 };
}

fn main2() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&nubmer_list);
    println!("The is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The is {}", result);
}

fn main1() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let result = largest(&nubmer_list);
    println!("The is {}", result);

    let nubmer_list = vec![34, 50, 25, 100, 65, 6000, 390];
    let result = largest(&nubmer_list);
    println!("The is {}", result);
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from{}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

// fn some_func<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { }

// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug { }
