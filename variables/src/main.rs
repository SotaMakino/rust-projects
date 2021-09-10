use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // '.'が見つかりませんでした
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn f<T>(x: &T) -> &T {
    return x;
}

fn longest_in_two<'x>(a: &'x str, b: &'x str) -> &'x str {
    if a.len() > b.len() {
        return a;
    }
    b
}

fn largest_char<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }

    &largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("{} is author", self.summarize_author())
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
        format!("author is: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    fn retweet() {}
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    // アナウンス！
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn return_sum(switch: bool) -> impl Summary {
    if (switch) {
        Tweet {
            username: "tw".to_string(),
            reply: false,
            retweet: false,
            content: "hoge".to_string(),
        }
    } else {
        Tweet {
            username: "tw".to_string(),
            reply: false,
            retweet: false,
            content: "hoge".to_string(),
        }
    }
}
