// RTRF


pub enum Rtrf<const N: usize>
{
    Constant(Constant<N>),
}


impl <const N: usize> Rtrf<N>
{
    pub fn ratio(&self, channel: usize) -> f64
    {
        match self
        {
            Rtrf::Constant(constant) => constant.ratio(channel)
        }
    }
}    


// Constant


pub struct Constant<const N: usize>
{
    ratio: [f64; N],
}


impl <const N: usize> Constant<N>
{
    pub fn new(ratio: [f64; N]) -> Constant<N>
    {
        for i in 0 .. N
        {
            if ratio[i] < 0. || ratio[i] > 1.
            {
                panic!("ratio[{}] parameter is out of range.", i);
            }
        }

        return Constant
        {
            ratio,
        };
    }


    pub fn ratio(&self, channel: usize) -> f64
    {
        return self.ratio[channel]
    }
}
