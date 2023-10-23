use std::fmt::{self, Display, Formatter, Result};

fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:          {}", 69420);
    println!("Base 2 (binary):  {:b}", 69420);
    println!("Base 8 (octal):   {:o}", 69420);
    println!("Base 16 (hex):    {:x}", 69420);
    println!("Base 16 (HEX):    {:X}", 69420);

    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");

    let pi = 3.141592;

    println!("Pi is roughly {pi:.2}");

    let val: Structure = Structure(123);
    println!("{}", val);

    let c1: Complex = Complex {
        real: 3.3,
        imag: 5.3,
    };
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1);

    let c2: Complex = Complex {
        real: 2.2,
        imag: -2.9,
    };
    println!("Display: {}", c2);
    println!("Debug: {:?}", c2);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
        println!("{}", color);
    }
}

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure val = '{}'", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            return write!(f, "{} + {}i", self.real, self.imag);
        }

        write!(f, "{} - {}i", self.real, -self.imag)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let vec = &self.0;

        write!(f, "[ ")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {

        write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue)
    }
}