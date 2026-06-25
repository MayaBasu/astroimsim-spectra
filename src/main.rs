use astroimsim_geometry::grid1d::GRID1D;
use crate::power_spectrum::SpectrumUnits;
use crate::power_spectrum::PowerSpectrum;
use crate::power_spectrum::SpectrumUnits::{f_lambda, F_nu};
use crate::visualize::{visulaize, STANDARD_SPECTRAL_GRID};

mod power_spectrum;
mod photonic_spectrum;
mod spectral_response;
mod visualize;



fn main() {
    /*
    let grid: GRID1D = GRID1D::new_empty(1.0,100.0,1000.0,0.01,1.0);

    let mut a = PowerSpectrum::flat(1.0,grid,"flat");

    a.convert_to(&F_nu);
    a.convert_to(&f_lambda);
    a.convert_to(&F_nu);
    a.convert_to(&f_lambda);

    a.convert_to(&f_lambda);

     */
    visulaize()



}