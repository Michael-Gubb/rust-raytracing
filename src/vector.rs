use core::fmt;


pub struct Vector3{
    x:f64,
    y:f64,
    z:f64
}

impl Vector3 {
    pub fn add(&self,other:Vector3)->Vector3{
        Vector3{
            x:self.x+other.x,
            y:self.y+other.y,
            z:self.z+other.z
        }
    }
    pub fn scale(&self,scaling_factor:f64)->Vector3{
        Vector3{
            x:self.x*scaling_factor,
            y:self.y*scaling_factor,
            z:self.z*scaling_factor
        }       
    }
    pub fn dot_product(&self,other:Vector3)->f64{
        self.x*other.x+self.y*other.y+self.z*other.z
    }    
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}, {}, {})",self.x,self.y,self.z)        
    }   
}


