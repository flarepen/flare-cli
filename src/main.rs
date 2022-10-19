use std::fs;
use std::path;

use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    input: path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct Rectangle {
    id: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Deserialize)]
struct Line {
    id: String,
    x: u32,
    y: u32,
    direction: Direction,
}

#[derive(Debug, Deserialize)]
struct Arrow {
    id: String,
    x: u32,
    y: u32,
    direction: Direction,
}

#[derive(Debug, Deserialize)]
struct Text {
    id: String,
    x: u32,
    y: u32,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Element {
    Rectangle(Rectangle),
    Line(Line),
    Arrow(Arrow),
    Text(Text),
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.input).expect("Cannot read input");
    let elem: Vec<Element> = serde_json::from_str(&content).expect("Bad json");

    println!("{:?}", elem);
}
