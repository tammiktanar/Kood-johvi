use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

impl Add for Object {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Object {
    type Output = Object;

    fn mul(mut self, scalar: f32) -> Self::Output {
        self.x *= scalar;
        self.y *= scalar;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> Self {
        Self {
            init_position,
            init_velocity,
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }

    fn integrate(&mut self) {
        self.actual_position = self.init_position + (self.init_velocity * self.time) + (GRAVITY * self.time.powi(2) * 0.5);
        self.actual_velocity = self.init_velocity + GRAVITY * self.time;

        self.actual_position.x = round_to(self.actual_position.x, 1);
        self.actual_position.y = round_to(self.actual_position.y, 1);
        self.actual_velocity.x = round_to(self.actual_velocity.x, 1);
        self.actual_velocity.y = round_to(self.actual_velocity.y, 1);
    }
}

fn round_to(f: f32, dec: i32) -> f32 {
    let mul = 10.0_f32.powi(dec);
    (f * mul).round() / mul
}

const GRAVITY: Object = Object{ x: 0.0, y: -9.8 };

impl Iterator for ThrowObject {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0;
        self.integrate();

        if self.actual_position.y >= 0.0 {
            Some(self.clone())
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
    }
}
