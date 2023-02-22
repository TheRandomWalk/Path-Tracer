use super::rtrf::Rtrf;
use super::rdf::Rdf;
use super::tdf::Tdf;


pub struct Material<const N: usize>
{
    pub rtrf:         Rtrf<N>,    // Reflection-transmission ratio function
    pub rdf:          Rdf<N>,     // Reflection distribution function
    pub tdf:          Tdf<N>,     // Transmission distribution function
    //pub transmission: [f64; N],   // Amount of light transmitter per unit of material
    //pub refraction:   [f64; N],   // Refraction index
    pub emission:     [f64; N],   // Emitted light
}


impl <const N: usize> Material<N>
{
    pub fn new(rtrf: Rtrf<N>, rdf: Rdf<N>, tdf: Tdf<N>, emission: [f64; N]) -> Material<N>
    {
        for i in 0 .. N
        {
        //     if transmission[i] < 0. || transmission[i] > 1.
        //     {
        //         panic!("transmission[{}] parameter out of range.", i);
        //     }

        //     if reflection[i] < 0. || reflection[i] > 1.
        //     {
        //         panic!("reflection[{}] parameter out of range.", i);
        //     }

        //     if refraction[i] < 0. || refraction[i] > 1.
        //     {
        //         panic!("refraction[{}] parameter out of range.", i);
        //     }

            if emission[i] < 0.
            {
                panic!("emission[{}] parameter out of range.", i);
            }
        }

        return Material
        {
            rtrf,
            rdf,
            tdf,
            // reflection,
            // transmission,
            // refraction,
            emission,
        };
    }
}


