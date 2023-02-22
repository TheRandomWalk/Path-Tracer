use std::f64::consts::PI;

use glam::DVec3 as Vec3;


pub enum Camera
{
    Projection(Projection),
}


impl Camera
{
    pub fn ray(&self, x: f64, y: f64) -> (Vec3, Vec3)
    {
        match self
        {
            Camera::Projection(projection) => projection.ray(x, y)
        }
    }
}


pub struct Projection
{
    position:     Vec3,     // Camera position
    view:         Vec3,     // Camera viewing vector direction
    up:           Vec3,     // Camera up vector direction
    right:        Vec3,     // Camera right vector direction
    fov:          f64,      // Horizontal FOV in degrees
    top_left:     Vec3,     // Top left position at 1 unit distance towards the view vector
    down_vec:     Vec3,     // Scaled down vector that advances the whole camera vertical axis
    right_vec:    Vec3,     // Scaled right vector that advances the whole camera horizontal axis
}


impl Projection
{
    pub fn new(position: Vec3, view: Vec3, up: Vec3, fov: f64, ratio: f64) -> Projection
    {
        let view      = view.normalize();
        let right     = view.cross(up).normalize();
        let up        = right.cross(view);

        let width     = 2. * (fov / 360. * PI).tan();
        let height    = width / ratio;

        let top_left  = view - right * (width * 0.5) + up * (height * 0.5);
        let right_vec = right * width;
        let down_vec  = -up   * height;

        let camera = Projection
        {
            position,
            view,
            up,
            right,
            fov,
            top_left,
            down_vec,
            right_vec,
        };

        return camera;
    }


    pub fn position(&self) -> Vec3
    {
        return self.position;
    }


    pub fn view(&self) -> Vec3
    {
        return self.view;
    }


    pub fn up(&self) -> Vec3
    {
        return self.up;
    }


    pub fn right(&self) -> Vec3
    {
        return self.right;
    }
    
    
    pub fn fov(&self) -> f64
    {
        return self.fov;
    }


    pub fn ray(&self, x: f64, y: f64) -> (Vec3, Vec3)
    {
        let direction = self.top_left + self.right_vec * x + self.down_vec * y;
        return (self.position, direction.normalize());
    }
}
