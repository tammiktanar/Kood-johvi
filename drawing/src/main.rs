

mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);


    // I've modified this a bit. Before it was giving Point struct references to 
    // the new function, but now it just gives the Point to it. This shouldn't
    // hopefully matter, though I am not a expert on references. If I did something
    // horribly wrong, feel free to change them back, though you'll have to change
    // the method signature. Else ignore this.
    let rectangle = gs::Rectangle::new(gs::Point::new(150, 150), gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        gs::Point::new(500, 500),
        gs::Point::new(250, 700),
        gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
