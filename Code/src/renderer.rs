use glam::DVec3 as Vec3;
use exr::prelude::*;
use smallvec::smallvec;

use super::config::Config;
use super::camera::Camera;
use super::object::Object;


pub struct Renderer<const N: usize>
{
    config:       Config<N>,
    camera:       Camera,    
    scene:        Vec<Object<N>>,
    image:        Vec<f64>,
    iteration:    usize,
}


impl<const N: usize> Renderer<N>
{
    pub fn new(config: Config<N>, camera: Camera, scene: Vec<Object<N>>) -> Renderer<N>
    {
        let image = vec![0.; config.width * config.height * config.channels];

        Renderer
        {
            config,
            camera,
            scene,  
            image,
            iteration: 0,
        }
    }

    
    pub fn render(&mut self, iterations: usize)
    {
        let dx = 1. / ((self.config.width  - 1) as f64);
        let dy = 1. / ((self.config.height - 1) as f64);

        for i in 0 .. iterations
        {
            let noise_x: f64 = (rand::random::<f64>() - 0.5) * dx;
            let noise_y: f64 = (rand::random::<f64>() - 0.5) * dy;

            for y in 0 .. self.config.height
            {
                let scaled_y = dy * (y as f64);
                
                for x in 0 .. self.config.width
                {
                    let scaled_x = dx * (x as f64);

                    for channel in 0 .. self.config.channels
                    {
                        let (position, direction) = self.camera.ray(scaled_x + noise_x, scaled_y + noise_y);
                        let light = self.trace(position, direction, channel);

                        self.image[((y * self.config.width) + x) * self.config.channels + channel] += light;
                    }
                }
            }
        }
        
        self.iteration += iterations;

    }


    pub fn iteration(&self) -> usize
    {
        return self.iteration;
    }


    // TODO: Support multispectral rendering
    pub fn rgb(&self, gamma: f64) -> Vec<u32>
    {
        let mut buffer = Vec::with_capacity(self.config.width * self.config.height);
        buffer.resize(self.config.width * self.config.height, 0u32);

        let mut max = 0.;

        for light in self.image.iter()
        {
            if max < *light
            {
                max = *light;
            }
        }

        for i in 0 .. self.config.width * self.config.height
        {
            let offset = i * 3;

            let r = (255. * (self.image[offset    ] / max).powf(gamma) + 0.5) as u32;
            let g = (255. * (self.image[offset + 1] / max).powf(gamma) + 0.5) as u32;
            let b = (255. * (self.image[offset + 2] / max).powf(gamma) + 0.5) as u32;

            buffer[i] = (r << 16) + (g << 8) + b;
        }

        return buffer;
    }
    

    pub fn save(&self, filename: &str)
    {
        let mut max = 0.;

        for intensity in self.image.iter()
        {
            if *intensity > max
            {
                max = *intensity;
            }
        }

        let scale = 1. / max;

        let mut r = vec![0.0; self.config.width * self.config.height];
        let mut g = vec![0.0; self.config.width * self.config.height];
        let mut b = vec![0.0; self.config.width * self.config.height];
        
        for i in 0 .. self.config.width * self.config.height
        {
            let offset = i * 3;

            r[i] = (self.image[offset    ] * scale) as f32;
            g[i] = (self.image[offset + 1] * scale) as f32;
            b[i] = (self.image[offset + 2] * scale) as f32;
        }

        let image = Image::from_layer
        (
            Layer::new
            (
                (self.config.width, self.config.height),
                LayerAttributes::named("main-rgb-layer"),
                Encoding::SMALL_LOSSLESS,
                AnyChannels::sort
                (
                    smallvec!
                    [
                        AnyChannel::new("R", FlatSamples::F32(r)),
                        AnyChannel::new("G", FlatSamples::F32(g)),
                        AnyChannel::new("B", FlatSamples::F32(b)),
                    ]
                ),
            )
        );
        
        image.write().to_file(filename).unwrap();
    }


    fn trace(&self, mut position: Vec3, mut direction: Vec3, channel: usize) -> f64
    {
        let mut path = Vec::new();    // f_n = (f_n-1 * a + b) * c
        let mut attenuation = Vec::new();
        

        for i in 0 .. self.config.path_length
        {
            let (distance, index) = self.intersect(position, direction);

            if distance == f64::INFINITY
            {
                break;
            }

            let mut dot = 1.;

            let object = &self.scene[index];

            if object.material.rtrf.ratio(channel) > rand::random()
            {
                let mut new_position = position + direction * distance;
                let normal = object.shape.normal(position, self.config.normal_epsilon);
                let mut new_direction = object.material.rdf.random(direction, normal);
                dot = new_direction.dot(normal);
                dot = dot.clamp(0.0, 1.0);

                position = new_position;
                direction = new_direction;
            }
            else 
            {
                //attenuation.push(object.material.tdf.emission(channel));
            }

            path.push
            (
                (
                    object.material.rdf.reflection(channel) * dot,
                    object.material.emission[channel],
                    (1. - attenuation.last().unwrap_or(&self.config.attenuation[channel])).powf(distance),
                )
            );
        }

        let mut light = 0.;

        for (a, b, c) in path.iter().rev()
        {
            light = (light * a + b) * c;
        }

        return light;
    }

    
    // TODO support intersections when ray originates from the inside of an object (dot(normal, direction) > 0 in those cases)
    fn intersect(&self, position: Vec3, direction: Vec3) -> (f64, usize)
    {
        let mut t = 0.;
        
        loop
        {
            let p = position + direction * t;
            let mut min_t = f64::INFINITY;
            let mut min_obj = 0;

            for (index, object) in self.scene.iter().enumerate()
            {
                let sdf = object.shape.sdf(p);
                
                if sdf < min_t
                {
                    if sdf > self.config.ray_accuracy || self.scene[index].shape.normal(position + direction * (t + sdf), self.config.normal_epsilon).dot(direction) < 0.
                    {
                        min_t = sdf;
                        min_obj = index;
                    }
                }
            }

            t += min_t;

            if t > self.config.ray_max
            {
                return (f64::INFINITY, 0)
            }

            if min_t < self.config.ray_accuracy
            {
                t += min_t * 0.5;
                return (t, min_obj);
            }
        }
    }
}



