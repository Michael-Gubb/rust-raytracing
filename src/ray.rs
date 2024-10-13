use crate::vector::Vector3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new_from_tuple(origin: (f64, f64, f64), direction: (f64, f64, f64)) -> Self {
        let origin = Vector3::new(origin.0, origin.1, origin.2);
        let direction = Vector3::new(direction.0, direction.1, direction.2);
        Self { origin, direction }
    }
    pub fn at(&self, time: f64) -> Vector3 {
        self.origin + self.direction * time
    }
    pub fn at_tuple(&self, time: f64) -> (f64, f64, f64) {
        self.at(time).into()
    }
}

mod test {
    use super::Ray;

    #[test]
    fn ray() {
        let initial_position = (0.1, 0.3, 0.2);
        let test_ray = Ray::new_from_tuple(initial_position, (0.1, 0.0, 0.0));
        let after_0_second = test_ray.at_tuple(0.0);
        assert_eq!(initial_position, after_0_second);
        let after_1_second = test_ray.at_tuple(1.0);
        assert_eq!((0.2, 0.3, 0.2), after_1_second);
    }
}
