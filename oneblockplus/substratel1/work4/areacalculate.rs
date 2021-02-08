fn main() {
    let circle = Circle {
        r: 5.0
    };
    let square = Square {
        x: 3.0,
        y: 4.0,
    };
    cal_shape(&circle);
    cal_shape(&square);
}

fn cal_shape<T: AreaCalculator>(shape: &T) {
    println!("{}", shape.area());
}


struct Circle {
    r: f32
}

struct Square {
    x: f32,
    y: f32,
}

impl AreaCalculator for Circle {
    fn area(&self) -> f32 {
        self.r * self.r * 3.1415926
    }
}

impl AreaCalculator for Square {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

pub trait AreaCalculator {
    fn area(&self) -> f32;
}
