use rand::Rng;
use raster::{Color, Image};

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(w: i32, h: i32) -> Point {
        let mut rng = rand::thread_rng();

        Point::new(
            rng.gen_range(0..=w),
            rng.gen_range(0..=h)
        )
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        img.display(self.x, self.y, random_color())
    }
}

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    pub fn random(w: i32, h: i32) -> Line {
        Line::new(Point::random(w, h), Point::random(w, h))
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        let given_color = random_color();
        draw_line(img, self.a.x, self.a.y, self.b.x, self.b.y, &given_color)
    }
}

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        let color = Color::rgb(255, 255, 255);

        draw_line(img, self.a.x, self.a.y, self.b.x, self.b.y, &color);
        draw_line(img, self.a.x, self.a.y, self.c.x, self.c.y, &color);
        draw_line(img, self.c.x, self.c.y, self.b.x, self.b.y, &color);
    }
}

pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        let color = Color::rgb(255, 255, 255);

        draw_line(img, self.a.x, self.b.y, self.b.x, self.b.y, &color);
        draw_line(img, self.a.x, self.b.y, self.a.x, self.a.y, &color);
        draw_line(img, self.b.x, self.a.y, self.a.x, self.a.y, &color);
        draw_line(img, self.b.x, self.a.y, self.b.x, self.b.y, &color);
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(w: i32, h: i32) -> Circle {
        let mut rng = rand::thread_rng();

        Circle::new(Point::random(w, h), rng.gen_range(0..=w))
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let given_color = &random_color();
        let mut x = self.radius;
        let mut y = 0;

        let mut re = x * x + y * y - self.radius * self.radius;
        while x >= y {
            // Set pixel
            img.display(self.center.x + x, self.center.y + y, Color::clone(given_color));
            img.display(self.center.x + y, self.center.y + x, Color::clone(given_color));

            img.display(self.center.x - x, self.center.y + y, Color::clone(given_color));
            img.display(self.center.x - y, self.center.y + x, Color::clone(given_color));

            img.display(self.center.x - x, self.center.y - y, Color::clone(given_color));
            img.display(self.center.x - y, self.center.y - x, Color::clone(given_color));

            img.display(self.center.x + x, self.center.y - y, Color::clone(given_color));
            img.display(self.center.x + y, self.center.y - x, Color::clone(given_color));

            if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
                re += 1 - 2 * x;
                x -= 1;
            }

            re += 2 * y + 1;
            y += 1;
        }
    }
}

// Traits
pub trait Drawable {
    fn draw(&self, _img: &mut Image) {}
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);

    Color::rgb(r, g, b)
}

fn draw_line(img: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, given_color: &Color) {
    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        // Set pixel
        img.display(x0, y0, given_color.clone());

        // Check end condition
        if x0 == x1 && y0 == y1 { break; };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}