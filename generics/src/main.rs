struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

struct Article {
    author: String,
    headline: String,
    _content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {
        format!("\"{}\" by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("\"{}\" ~{}", self.content, self.username)
    }
}

struct Blog {
    username: String,
    _content: String,
}

impl Summary for Blog {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn print_summary<T: Summary>(text: &T) {
    println!("{}", text.summarize())
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let m = largest(&arr);
    println!("The largest element of {:?} is {}", arr, m);

    let float = Point { x: 1.3, y: 2.6 };
    let int = Point { x: 69, y: 420 };
    println!("Int: ({}, {})", int.x(), int.y);
    println!("Float: ({}, {})", float.x, float.y);
    println!("Float dist from origin: {}", float.distance_from_origin());
    // This errors because method is only implemented for Point<f64>
    // println!("Int dist from origin: {}", int.distance_from_origin());

    let tweet = Tweet {
        username: String::from("bendermeister"),
        content: String::from("i like wearing womens underwear"),
    };

    let blog = Blog {
        username: String::from("blogger"),
        _content: String::from("i would give anything for a job"),
    };

    let article = Article {
        author: String::from("Martin Luther"),
        headline: String::from("The Bible is Kinda Meh"),
        _content: String::from("So ladies and gentleman lets talk about taxes,...."),
    };

    print_summary(&tweet);
    print_summary(&article);
    print_summary(&blog);
}
