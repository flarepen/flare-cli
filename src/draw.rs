use crate::element;

const LEFT_TOP: char = '┌';
const RIGHT_TOP: char = '┐';
const LEFT_BOTTOM: char = '└';
const RIGHT_BOTTOM: char = '┘';
const HORIZONTAL: char = '─';
const VERTICAL: char = '│';
const ARROW_LEFT: char = '◀';
const ARROW_RIGHT: char = '▶';
const ARROW_UP: char = '▶';
const ARROW_DOWN: char = '▼';
const SPACE: char = ' ';

type Shape = Vec<Vec<char>>;

#[derive(Debug)]
struct Bounds {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

pub fn draw_elements(elements: &Vec<element::Element>) {
    let bounds = bounds(elements);

    let mut merged: Shape = Vec::with_capacity(bounds.height);

    for _ in 1..bounds.height {
        merged.push(row(bounds.width, SPACE, SPACE, SPACE));
    }

    for element in elements {
        let shape = shape(element);
        let offset_x = x(element) - bounds.x;
        let offset_y = y(element) - bounds.y;

        for (row_num, row) in shape.iter().enumerate() {
            for (col_num, ch) in row.iter().enumerate() {
                if *ch != SPACE {
                    merged[row_num + offset_y][col_num + offset_x] = *ch;
                }
            }
        }
    }

    println!(
        "{}",
        merged
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn bounds(elements: &Vec<element::Element>) -> Bounds {
    let x_min = elements.iter().map(|e| x(e)).min().unwrap_or(0);
    let x_max = elements.iter().map(|e| x(e) + width(e)).max().unwrap_or(0);
    let y_min = elements.iter().map(|e| y(e)).min().unwrap_or(0);
    let y_max = elements.iter().map(|e| y(e) + height(e)).max().unwrap_or(0);

    let width = x_max - x_min;
    let height = y_max - y_min;

    return Bounds {
        x: x_min,
        y: y_min,
        width,
        height,
    };
}

fn x(element: &element::Element) -> usize {
    match element {
        element::Element::Rectangle(val) => val.x,
        element::Element::Line(val) => val.x,
        element::Element::Arrow(val) => val.x,
        element::Element::Text(val) => val.x,
    }
}

fn y(element: &element::Element) -> usize {
    match element {
        element::Element::Rectangle(val) => val.y,
        element::Element::Line(val) => val.y,
        element::Element::Arrow(val) => val.y,
        element::Element::Text(val) => val.y,
    }
}

fn width(element: &element::Element) -> usize {
    match element {
        element::Element::Rectangle(val) => val.width,
        element::Element::Line(val) => match val.direction {
            element::Direction::Left | element::Direction::Right => val.len,
            element::Direction::Down | element::Direction::Up => 1,
        },
        element::Element::Arrow(val) => match val.direction {
            element::Direction::Left | element::Direction::Right => val.len,
            element::Direction::Down | element::Direction::Up => 1,
        },
        element::Element::Text(val) => val.content.len(),
    }
}

fn height(element: &element::Element) -> usize {
    match element {
        element::Element::Rectangle(val) => val.height,
        element::Element::Line(val) => match val.direction {
            element::Direction::Left | element::Direction::Right => 1,
            element::Direction::Down | element::Direction::Up => val.len,
        },
        element::Element::Arrow(val) => match val.direction {
            element::Direction::Left | element::Direction::Right => 1,
            element::Direction::Down | element::Direction::Up => val.len,
        },
        element::Element::Text(_) => 1,
    }
}

fn shape(element: &element::Element) -> Shape {
    match element {
        element::Element::Rectangle(val) => rectangle(val),
        element::Element::Line(val) => line(val),
        element::Element::Arrow(val) => arrow(val),
        element::Element::Text(val) => text(val),
    }
}

fn row(width: usize, left: char, inside: char, right: char) -> Vec<char> {
    let mut row: Vec<char> = Vec::with_capacity(width);
    row.push(left);
    for _ in 1..width - 2 {
        row.push(inside)
    }
    row.push(right);

    return row;
}

fn rectangle(rectangle: &element::Rectangle) -> Shape {
    let mut shape: Shape = Vec::with_capacity(rectangle.height);

    shape.push(row(rectangle.width, LEFT_TOP, HORIZONTAL, RIGHT_TOP));
    for _ in 1..rectangle.height - 2 {
        shape.push(row(rectangle.width, VERTICAL, SPACE, VERTICAL));
    }
    shape.push(row(rectangle.width, LEFT_BOTTOM, HORIZONTAL, RIGHT_BOTTOM));

    return shape;
}

fn line(line: &element::Line) -> Shape {
    match line.direction {
        element::Direction::Left | element::Direction::Right => {
            return vec![row(line.len, HORIZONTAL, HORIZONTAL, HORIZONTAL)]
        }
        element::Direction::Down | element::Direction::Up => {
            let mut shape: Shape = Vec::with_capacity(line.len);

            for _ in 1..line.len {
                shape.push(vec![VERTICAL]);
            }

            return shape;
        }
    }
}

fn arrow(arrow: &element::Arrow) -> Shape {
    match arrow.direction {
        element::Direction::Left => {
            return vec![row(arrow.len, ARROW_LEFT, HORIZONTAL, HORIZONTAL)]
        }
        element::Direction::Right => {
            return vec![row(arrow.len, HORIZONTAL, HORIZONTAL, ARROW_RIGHT)]
        }
        element::Direction::Down => {
            let mut shape: Shape = Vec::with_capacity(arrow.len);

            for _ in 1..arrow.len - 1 {
                shape.push(vec![VERTICAL]);
            }

            shape.push(vec![ARROW_DOWN]);

            return shape;
        }
        element::Direction::Up => {
            let mut shape: Shape = Vec::with_capacity(arrow.len);

            shape.push(vec![ARROW_UP]);

            for _ in 1..arrow.len - 1 {
                shape.push(vec![VERTICAL]);
            }

            return shape;
        }
    }
}

fn text(text: &element::Text) -> Shape {
    return vec![text.content.chars().collect::<Vec<char>>()];
}
