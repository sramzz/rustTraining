use std::fmt;
struct Circle{
    radius: f32
}
struct Square{
    side:f32
}
struct Rectangle{
    width: f32,
    height: f32
}
struct Triangle{
    base:f32,
    height:f32
}
enum Shape {
    Circle(Circle),
    Square(Square),
    Triangle(Triangle),
    Rectangle(Rectangle)
}
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(c) => write!(f, "Circle with radius {}", c.radius),
            Shape::Square(s) => write!(f, "Square with side {}", s.side),
            Shape::Rectangle(r) => write!(f, "Rectangle with width {} and height {}", r.width, r.height),
            Shape::Triangle(t) => write!(f, "Triangle with base {} and height {}", t.base, t.height),
        }
    }
}
impl Shape {
    fn area(&self) -> f32{
        match self {
            Shape::Circle(c) => std::f32::consts::PI * c.radius * c.radius,
            Shape::Rectangle(r) => r.width * r.height,
            Shape::Triangle(t) => t.base * t.height,
            Shape::Square(s) => s.side
        }
    }
}

fn main() {
    let shapes_vector: Vec<Shape> = vec![
        Shape::Circle(Circle{radius:2.5}),
        Shape::Square(Square{side:4.0}),
        Shape::Rectangle(
            Rectangle{
                width: 4.0,
                height: 9.0
        }),
        Shape::Triangle(Triangle{
            base: 5.0,
            height:3.0
        })
    ];
    for shape in shapes_vector.iter(){
        println!("the area of {} is {}",shape,shape.area());
    
    let _hola: &str = "hola";
    }
    //Learning to use the Option Enum
    let x:i32 = 5;
    let y:Option<i32> = Some(5);
    let sum = x + y.unwrap_or(0) ;
    print!("\nthe sum was {}",sum)

}
