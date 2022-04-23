struct Article {
    name: String,
    length: i32,
}

pub fn run() {
    let article = Article{
        name: String::from ("hello"),
        length: 32,
    };

    println!("article name {}, length = {}", article.name, article.length);
}