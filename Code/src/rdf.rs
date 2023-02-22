use std::f64::consts::PI;

use glam::DVec3 as Vec3;


// RDF

pub enum Rdf<const N: usize>
{
    None,
    Diffuse(Diffuse<N>),
}


impl <const N: usize> Rdf<N>
{
    pub fn random(&self, in_vector: Vec3, normal: Vec3) -> Vec3
    {
        match self
        {
            Rdf::None             => panic!("None does not implement random()."),
            Rdf::Diffuse(diffuse) => diffuse.random(in_vector, normal),
        }
    }


    pub fn pdf(&self, in_vector: Vec3, out_vector: Vec3, normal: Vec3) -> f64
    {
        match self
        {
            Rdf::None             => panic!("None does not implement pdf()."),
            Rdf::Diffuse(diffuse) => diffuse.pdf(in_vector, out_vector),
        }
    }


    pub fn reflection(&self, channel: usize) -> f64
    {
        match self
        {
            Rdf::None             => panic!("None does not implement reflection()."),
            Rdf::Diffuse(diffuse) => diffuse.reflection(channel),
        }
    }
}


// Diffuse


pub struct Diffuse<const N: usize>
{
    reflection: [f64; N],
}


impl <const N: usize> Diffuse<N>
{
    pub fn new(reflection: [f64; N]) -> Diffuse<N>
    {
        for i in 0 .. N
        {
            if reflection[i] < 0. || reflection[i] > 1.
            {
                panic!("reflection[{}] parameter is out of range.", i);
            }
        }

        return Diffuse
        {
            reflection
        };
    }


    pub fn random(&self, in_vector: Vec3, normal: Vec3) -> Vec3
    {
        loop
        {
            let x: f64 = rand::random::<f64>() * 2. - 1.;
            let y: f64 = rand::random::<f64>() * 2. - 1.;
            let z: f64 = rand::random::<f64>() * 2. - 1.;

            if x != 0. || y != 0. || z != 0.
            {
                let mut v = Vec3::new(x, y, z);
                let r2 = v.length_squared();

                if r2 <= 1.
                {
                    v = v / r2.sqrt();

                    if v.dot(normal) > 0.
                    {
                        return v;
                    }
                    else
                    {
                        return -v;
                    }
                }
            }
        }
    }


    pub fn pdf(&self, in_vector: Vec3, out_vector: Vec3) -> f64
    {
        return 1. / (2. * PI);
    }


    pub fn reflection(&self, channel: usize) -> f64
    {
        return self.reflection[channel];
    }
}
