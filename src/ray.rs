use crate::vector;


#[derive(Debug, Clone, Copy, Default)]
pub struct Ray{
    origin:vector::Vector3,
    direction:vector::Vector3
}

impl Ray{
    pub fn new_from_tuple(origin:(f64,f64,f64),direction:(f64,f64,f64))->Self{
        let origin = vector::Vector3{x:origin.0,y:origin.1,z:origin.2};
        let direction = vector::Vector3{x:direction.0,y:direction.1,z:direction.2};
        Self{origin,direction}
    }
    pub fn at(&self,time:f64)->vector::Vector3 {
        self.origin + self.direction*time
    }
}

mod test{
    use super::Ray;

    #[test]
    fn ray(){
        let initial_position = (0.1,0.3,0.2);
        let test_ray = Ray::new_from_tuple(initial_position,(0.1,0.0,0.0));
        let after_0_second = test_ray.at(0.0);
        assert_eq!(initial_position,after_0_second.into());
        let after_1_second = test_ray.at(1.0);
        assert_eq!((0.2,0.3,0.2),after_1_second.into());       
    }
}