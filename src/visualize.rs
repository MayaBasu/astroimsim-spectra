use astroimsim_geometry::grid1d::GRID1D;
use egui::Response;
use crate::spectral_response::SpectralResponseCurve;

pub fn visulaize(){


    pub const STANDARD_SPECTRAL_GRID: GRID1D = GRID1D::new_empty(0.5,100.0,1000.0,0.01,1.0);

    const FUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_contamination.dat";
    const NUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_contamination.dat";
    const FUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_filter_response.dat";
    const NUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_filter_response.dat";
    const NUV_QE_CURVE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_NUV_QE.dat";
    const DICHROIC_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_dichroic_response.dat";


    pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
    pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
    pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
    pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(0.5,120.5,1100.0,0.01,1000.0);


    let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
    let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
    let mut FUV_RESPONSE: SpectralResponseCurve = SpectralResponseCurve::new("FUV_RESPONSE",FUV_RESPONSE_GRID,FUV_RESPONSE_PATH);
    let NUV_RESPONSE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_RESPONSE",NUV_RESPONSE_GRID,NUV_RESPONSE_PATH);
    let  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE",NUV_QE_CURVE_GRID,NUV_QE_CURVE_PATH);
    let DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("DICHROIC",DICHROIC_GRID,DICHROIC_PATH);
    
    

    FUV_RESPONSE.write_to_dat("fuv","fuv");
    println!("{:?}",FUV_RESPONSE)




}