use super::shape::Shape;
use super::material::Material;


pub struct Object<const N: usize>
{
    pub shape:    Shape,
    pub material: Material<N>,
}


impl <const N: usize> Object<N>
{
    pub fn new(shape: Shape, material: Material<N>) -> Object<N>
    {
        return Object
        {
            shape,
            material,
        };
    }
}
