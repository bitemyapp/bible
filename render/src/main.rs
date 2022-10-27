use askama::Template; // bring trait in scope

#[derive(Template)]
#[template(path = "toc.html")]
struct TocTemplate {
}

fn main() {
    let toc = TocTemplate {};
    println!("{}", toc.render().unwrap());
}
