#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Point
impl Point {
    pub fn distance(self, p2: &Point) -> f64 {
        let st_val = (self.x - p2.x).powf(2.0);
        let nd_val = (self.y - p2.y).powf(2.0);
        return (st_val + nd_val).sqrt()
    }
}

// Circle

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {center: Point{x: x, y: y}, radius: radius}
    }

    pub fn diameter(self) -> f64 {
        

        return self.radius * 2.0
    }

    pub fn area(self) -> f64 {
        let pi = std::f64::consts::PI;

        return pi * self.radius.powf(2.0)
    }

    pub fn intersect(self, other: &Circle) -> bool {
        return self.center.distance(&other.center) < (self.radius + other.radius)
    }
}