#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let given_area = x * y;

    let object_area = times * match objects {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a).ceil() as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b).ceil() as usize,
    };

    object_area <= given_area
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let given_area = x * y * z;

    let object_volume = times * match objects {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Sphere => sphere_volume(a).ceil() as usize,
        GeometricalVolumes::Cone => cone_volume(a, b).ceil() as usize,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b).ceil() as usize,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),
    };

    object_volume <= given_area
}