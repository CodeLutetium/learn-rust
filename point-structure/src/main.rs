#[derive(Debug)]

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: std::fmt::Display + std::clone::Clone, U: std::fmt::Display + std::clone::Clone> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {
            x: x,
            y: y
        }
    }
    
    fn print_point(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    fn swap_points(&self) -> Point<U, T> {
        Point {
            x: self.y.clone(),
            y: self.x.clone()
        }
    }
}

fn main() {
    let a = Point::new(12, 45.3);
    let b = a.swap_points();
    a.print_point();
    b.print_point();
}
