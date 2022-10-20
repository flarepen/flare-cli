use std::fs;
use std::path;

use clap::Parser;

mod draw;
mod element;

use element::Element;
use draw::draw_elements;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    input: path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.input).expect("Cannot read input");
    let elements: Vec<Element> = serde_json::from_str(&content).expect("Bad json");

    draw_elements(&elements);
}
