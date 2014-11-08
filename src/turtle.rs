#![feature(globs)]

use std::num::*;

pub struct Turtle {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub pen: bool
}


impl Turtle {

    pub fn forward(&self, dist: f32) -> Turtle { self.move_self(dist) }
    pub fn backward(&self, dist: f32) -> Turtle { self.move_self(-dist) }

    fn move_self(&self, distance: f32) -> Turtle {
        let coefficients = self.angle.to_radians().sin_cos();
        Turtle { 
            x: (coefficients.val1() * distance) + self.x,
            y: (coefficients.val0() * distance) + self.y,
            angle: self.angle,
            pen: self.pen
        }
    }

    pub fn right(&self, angle: f32) -> Turtle { self.rotate_self(angle) }
    pub fn left(&self, angle: f32) -> Turtle { self.rotate_self(-angle) }
    fn rotate_self(&self, angle: f32) -> Turtle {
        let new_angle = (360.0 + (self.angle + angle) % 360.0) % 360.0;
        Turtle {
            x: self.x,
            y: self.y,
            angle: new_angle,
            pen: self.pen
        }
    }

    pub fn log(&self) -> Turtle {
        println!("x:{}\ny:{}\nangle:{}\npen:{}", 
                self.x,
                self.y,
                self.angle,
                self.pen)
        *self

    }
}

fn main() {
    let t = example_turtle();
    t.log()
     .right(90.0)
     .log()
     .right(90.0)
     .log()
     .right(90.0)
     .log()
     .right(90.0)
     .log()
     .right(90.0)
     .log()
     .left(90.0)
     .log()
     .left(90.0)
     .log()
     .left(90.0)
     .log()
     .left(90.0)
     .log()
     .left(90.0);
}
// fn main() {
//     let t = example_turtle();
//     t.log()
//      .forward(10.0)
//      .log()
//      .right(120.0)
//      .log()
//      .forward(10.0)
//      .log()
//      .right(220.0)
//      .log()
//      .right(140.0)
//      .log()
//      .backward(10.0)
//      .log()
//      .left(120.0)
//      .log()
//      .backward(10.0)
//      .log();
// }

// x: 20        y: 20        angle: 135 pen: true
// x: 12.928932 y: 27.071068 angle: 135 pen: true
// x: 12.928932 y: 27.071068 angle: 255 pen: true
// x: 10.340742 y: 17.411808 angle: 255 pen: true
// x: 10.340742 y: 17.411808 angle: 115 pen: true
// x: 10.340742 y: 17.411808 angle: 255 pen: true
// x: 12.928932 y: 27.071068 angle: 255 pen: true
// x: 12.928932 y: 27.071068 angle: 135 pen: true
// x: 20        y: 20        angle: 135 pen: true

pub fn example_turtle() -> Turtle {
    Turtle { x: 20.0, y: 20.0, angle: 90.0, pen: true }
}

#[test]
fn test_forward() {
    let subject = example_turtle().forward(10.0);
    let expected = Turtle { x: 12.928932, y: 27.071068, angle: 135.0, pen: true };
    assert_eq!(subject.x, expected.x)
    assert_eq!(subject.y, expected.y)
}

#[test]
fn test_backward() {
    let t = Turtle { x: 12.928932, y: 27.071068, angle: 135.0, pen: true };
    let subject = t.backward(10.0);
    let expected = example_turtle();
    assert_eq!(subject.x, expected.x)
    assert_eq!(subject.y, expected.y)
}

#[test]
fn test_right() {
    let subject = example_turtle().right(120.0);
    let expected_angle = 255.0;
    assert_eq!(subject.angle, expected_angle)
}

#[test]
fn test_left() {
    let subject = example_turtle().left(120.0);
    let expected_angle = 15.0;
    assert_eq!(subject.angle, expected_angle)
}
