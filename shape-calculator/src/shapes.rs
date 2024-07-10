use std::f64::{consts::PI, INFINITY};

pub trait Calculate {
    fn calculate_area(&self) -> f64;
    fn calculate_perimeter(&self) -> f64;
}

pub struct Shape {
    color: String

}

impl Shape {
    pub fn new(color: String) -> Self {
        Shape {
            color: color
        }
    }
    
    pub fn print_shape(&self) {
        println!("The color of this shape is {:?}.", self.color);
    }
}

pub struct Circle {
    shape: Shape,
    radius: f64
}

impl Circle {
    pub fn new(color: String, radius: f64) -> Self {
        Circle {
            shape: Shape::new(color),
            radius: radius
        }
    }
}

impl Calculate for Circle {
    fn calculate_area(&self) -> f64 {
        PI * self.radius.powf(2.0)
    }

    fn calculate_perimeter(&self) -> f64 {
        PI * 2.0 * self.radius
    }
}