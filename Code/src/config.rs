pub struct Config<const N: usize>
{
    pub width:          usize,     // Image width
    pub height:         usize,     // Image height
    pub channels:       usize,     // Number of channels
    pub ray_accuracy:   f64,       // Ray accuracy
    pub ray_max:        f64,       // Ray max length
    pub normal_epsilon: f64,       // Epsilon to compute the shape normal vector
    pub path_length:    usize,     // Maximum path length
    pub attenuation:    [f64; N],  // Attenuation per unit distance
}
