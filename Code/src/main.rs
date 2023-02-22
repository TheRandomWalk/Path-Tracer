use std::time::Instant;

use glam::DVec3 as Vec3;

use lux::renderer::Renderer;
use lux::config::Config;
use lux::camera::{Camera, Projection};
use lux::object::Object;
use lux::shape::{Shape, Sphere};
use lux::material::Material;
use lux::rtrf::{Rtrf, Constant};
use lux::rdf::{Rdf, Diffuse};
use lux::tdf::{Tdf};


fn main() 
{
    const WIDTH:  usize = 3840 * 2;
    const HEIGHT: usize = 2160 * 2;

    let config = Config
    {
        width:          WIDTH,
        height:         HEIGHT,
        channels:       3,
        ray_accuracy:   1E-7,
        ray_max:        1E3,
        normal_epsilon: 1E-7,
        path_length:    10,
        attenuation:    [0., 0., 0.],
    };

    let camera = Camera::Projection
    (
        Projection::new
        (
            Vec3::new(0., 3., 0.), 
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 1., 0.), 
            60.,
            16. / 9.,
        )
    );

    let mut scene = Vec::new();

    scene.push
    (
        Object::new
        (
            Shape::Sphere(Sphere::new(Vec3::new(-6., 1., -20.), 1.)),
            Material::new
            (
                Rtrf::Constant(Constant::new([1., 1., 1.])),
                Rdf::Diffuse(Diffuse::new([0.1, 0.1, 1.])),
                Tdf::None,
                [0., 0., 0.],
            )
        )
    );

    scene.push
    (
        Object::new
        (
            Shape::Sphere(Sphere::new(Vec3::new(0., 3., -20.), 3.)),
            Material::new
            (
                Rtrf::Constant(Constant::new([1., 1., 1.])),
                Rdf::Diffuse(Diffuse::new([1., 1., 1.])),
                Tdf::None,
                [1., 1., 1.],
            )
        )
    );

    scene.push
    (
        Object::new
        (
            //Shape::Sphere(Sphere::new(Vec3::new(6., 1., -10.), 1.)),
            Shape::Sphere(Sphere::new(Vec3::new(6., 2., -20.), 2.)),
            Material::new
            (
                Rtrf::Constant(Constant::new([1., 1., 1.])),
                Rdf::Diffuse(Diffuse::new([1., 0.1, 0.1])),
                Tdf::None,
                [0., 0., 0.],
            )
        )
    );

    scene.push
    (
        Object::new
        (
            Shape::Sphere(Sphere::new(Vec3::new(0., -1000., -20.), 1000.)),
            Material::new
            (
                Rtrf::Constant(Constant::new([1., 1., 1.])),
                Rdf::Diffuse(Diffuse::new([0.75, 0.75, 0.75])),
                Tdf::None,
                [0., 0., 0.],
            )
        )
    );

    let mut renderer = Renderer::new(config, camera, scene);

    let time = Instant::now();

    loop
    {
        renderer.render(10);
        renderer.save("output.exr");
        println!("[{:.1} h] Iterations: {}", time.elapsed().as_secs_f64() / 3600., renderer.iteration());
    }

}
