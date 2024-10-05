fn main() {
    let program = match std::env::args().nth(0) {
        Some(p) => p,
        None => String::from("pdoc"),
    };
    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: {} <file.md>", program);
            std::process::exit(1);
        }
    };

    let content = std::fs::read_to_string(&filename)
        .expect(&format!("Failed to read file [{}]", filename));

    let html = markdown::to_html_with_options(&content, &markdown::Options::gfm()).expect("Failed to render html");

    println!("{}", html);
}
