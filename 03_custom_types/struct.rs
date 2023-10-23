#!  [allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Uint;

// A tuple struct
struct Pair (i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {top_left: Point {x: x1, y: y1}, bottom_right: Point {x: x2, y: y2}} = rect;

    (x2 - x1) * (y2 - y1)
}

fn square(top_left: Point, size: f32) -> Rectangle {
    let Point {x: x1, y: y1} = top_left;

    Rectangle {
        top_left: top_left,
        bottom_right: Point {x: x1 + size, y: y1 + size},
    }
}

fn main () {
    let name = String::from("Peter");
    let age: u8 = 27;

    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point{x: 5.2, ..point};

    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point{x :left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        top_left: Point{x:left_edge, y: top_edge + 1.0},
        bottom_right,
    };

    println!("rectangle area: {}", rect_area(&rectangle));

    println!("square: {:?}", square(point, 5.0));

    let _unit = Uint;

    let pair = Pair(1, 1.2);

    println!("pair contains: {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains: {:?} and {:?}", integer, decimal);
}