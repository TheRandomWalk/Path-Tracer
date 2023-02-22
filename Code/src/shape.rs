use glam::DVec3 as Vec3;

use std::fmt;


// Shape


#[derive(Clone)]
pub enum Shape
{
    Sphere(Sphere),
}


impl Shape
{
    pub fn sdf(&self, position: Vec3) -> f64
    {
        match self
        {
            Shape::Sphere(sphere) => sphere.sdf(position)
        }
    }
    
    
    pub fn normal(&self, position: Vec3, epsilon: f64) -> Vec3
    {
        match self
        {
            Shape::Sphere(sphere) => sphere.normal(position, epsilon)
        }
    }
}


impl fmt::Display for Shape
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Shape::Sphere(sphere) => write!(f, "Sphere: [c: ({}, {}, {}); r: {}]", sphere.position.x, sphere.position.y, sphere.position.z, sphere.radius)
        }
    }
}
    

// Sphere


#[derive(Clone)]
pub struct Sphere
{
    position: Vec3,     // Sphere position
    radius:   f64,      // Sphere radius
}


impl Sphere
{
    pub fn new(position: Vec3, radius: f64) -> Sphere
    {
        return Sphere
        {
            position,
            radius,
        };
    }


    pub fn sdf(&self, position: Vec3) -> f64
    {
        return (position - self.position).length() - self.radius;
    }


    pub fn normal(&self, position: Vec3, epsilon: f64) -> Vec3
    {
        let x = self.sdf(position + Vec3::new(epsilon, 0., 0.)) - self.sdf(position + Vec3::new(-epsilon, 0., 0.));
        let y = self.sdf(position + Vec3::new(0., epsilon, 0.)) - self.sdf(position + Vec3::new(0., -epsilon, 0.));
        let z = self.sdf(position + Vec3::new(0., 0., epsilon)) - self.sdf(position + Vec3::new(0., 0., -epsilon));

        return Vec3::new(x, y, z).normalize();
    }
}
