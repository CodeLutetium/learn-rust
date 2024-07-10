use crate::shapes::*;

pub mod shapes;


fn main() {
    let s1: Shape = Shape::new("Green".to_string());
    s1.print_shape();

    let s2: Circle = Circle::new("Blue".to_string(), 2.0);
    let area = s2.calculate_area();
    println!("Area of s2 is {area}");
    s2.shape.print_shape();
}
