extern crate scraper;

use std::io::{self, Read, Write};
use std::fs::File;
use scraper::{Selector, Html};

fn main() {
	let testfile = "./test_input/messages.html";

    let selector_string = "div.message";
    let selector = Selector::parse(&selector_string).unwrap();

    //write!(stdout, "HTML file:\n").unwrap();
    //stdout.flush().unwrap();
    //input.clear();
    //stdin.read_to_string(&mut input).unwrap();

    let mut f = File::open(&testfile).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let document = Html::parse_document(&contents);

    for node in document.select(&selector) {
        println!("{:?}", node.value());
    }
}
