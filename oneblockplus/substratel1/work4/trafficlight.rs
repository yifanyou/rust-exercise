fn main() {
    let mut light = TrafficLight::Red;
    println!("red light time is {}", light.time());
    
    light = TrafficLight::Green;
    println!("green light time is {}", light.time());
    
    light = TrafficLight::Yellow;
    println!("yellow light time is {}", light.time());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait TimeDuration {
    fn time(&self) -> u8;
}

impl TimeDuration for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 5,
        }
    }
}
