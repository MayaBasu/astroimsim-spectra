use astroimsim_geometry::grid1d::GRID1D;
use egui::Response;
use crate::power_spectrum::{PowerSpectrum, SpectrumUnits};
use crate::power_spectrum::SpectrumUnits::{F_lambda, F_nu};
use crate::spectral_response::SpectralResponseCurve;
pub const STANDARD_SPECTRAL_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);

pub fn visulaize(){



    const FUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_contamination.dat";
    const NUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_contamination.dat";
    const FUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_filter_response.dat";
    const NUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_filter_response.dat";
    const NUV_QE_CURVE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_QE.dat";
    const DICHROIC_PATH: &'static str = "/Users/mayabasu/RustroverProjects/astroimsim-spectra/src/spectral_response_files/UVIM_dichroic_response.dat";
    const MIRROR_PATH: &'static str = "src/spectral_response_files/mirror_reflectivity.dat";

    pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
    pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
    pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
    pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1000.0,0.01,1000.0);
    pub const MIRROR_GRID:GRID1D = GRID1D::new_empty(1.0,110.0,1100.0,0.01,1.0);

  //  let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
   // let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
    let mut FUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("FUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,1,"      ");
    let mut NUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("NUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,2,"      ");
    let mut  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE",NUV_QE_CURVE_GRID,NUV_QE_CURVE_PATH,1,"   ");
    let mut FUV_CONTAMINATION:SpectralResponseCurve = SpectralResponseCurve::new("FUV Contamination",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH,1,"   ");
    FUV_CONTAMINATION.write_to_dat("contamination","FUV contamination");
    let mut NUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("NUV Filter response",NUV_RESPONSE_GRID,NUV_RESPONSE_PATH,1,"   ");
    let mut FUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("FUV Filter response",FUV_RESPONSE_GRID,FUV_RESPONSE_PATH,1,"   ");

    NUV_FILTER_CURVE.write_to_dat("nuv_filter_curve","NUV Filter curve");
    FUV_FILTER_CURVE.write_to_dat("fuv_filter_curve","FUV Filter curve");
    NUV_QE.write_to_dat("nuv_qe_curve","NUV QE curve");

    let mut NUV_FILTER_QE = SpectralResponseCurve::compose(vec![NUV_FILTER_CURVE,NUV_QE]);
    NUV_FILTER_QE.write_to_dat("nuv_filter_qe","NUV filter cuve + NUV QE");


      //let DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("DICHROIC",DICHROIC_GRID,DICHROIC_PATH);
    let mut MIRROR_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("Mirror", MIRROR_GRID, MIRROR_PATH,1,"    ");
    MIRROR_CURVE.re_grid(&STANDARD_SPECTRAL_GRID);
    let mut MIRROR_CURVE_3 = MIRROR_CURVE.clone();
    MIRROR_CURVE_3.self_compose(3);
    MIRROR_CURVE_3.re_grid(&STANDARD_SPECTRAL_GRID);


    FUV_DICHROIC.write_to_dat("fuvdichroic","fuv path of dichroic (transmission)");
    NUV_DICHROIC.write_to_dat("nuvdichroic","nuv path (reflection)");

    let mut input_spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum");

    input_spectrum.convert_to(&SpectrumUnits::f_lambda);
    input_spectrum.write_to_dat("input","initial spectrum");

    MIRROR_CURVE.write_to_dat("mirror","mirror response curve");
    MIRROR_CURVE_3.write_to_dat("mirror3","3 mirror response curves composed together");

    input_spectrum.apply_spectral_response(&MIRROR_CURVE_3);
    input_spectrum.write_to_dat("input_plus_mirrors","Input spectrum with the mirror curve applied three times");
    let mut input_spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum");
    input_spectrum.convert_to(&SpectrumUnits::f_lambda);
    let mut fuv_path = input_spectrum.clone();
    let mut nuv_path = input_spectrum;
    fuv_path.apply_spectral_response(&MIRROR_CURVE_3);
    fuv_path.apply_spectral_response(&FUV_DICHROIC);
    fuv_path.write_to_dat("fuv_dic_mir","Input spectrum with FUV and 3 mirror curves ");
    fuv_path.apply_spectral_response(&FUV_FILTER_CURVE);
    fuv_path.write_to_dat("fuv_dic_mir_qe","Input spectrum with FUV and 3 mirror curves  abd QE");


    nuv_path.apply_spectral_response(&MIRROR_CURVE_3);
    nuv_path.apply_spectral_response(&NUV_DICHROIC);
    nuv_path.write_to_dat("nuv_dic_mir","Input spectrum with NUV and 3 mirror curves ");
    nuv_path.apply_spectral_response(&NUV_FILTER_QE);
    nuv_path.write_to_dat("nuv_dic_mir_qe","Input spectrum with NUV and 3 mirror curves  abd QE");

    // println!("{:?}",input_spectrum)

    nuv_path.apply_spectral_response(&FUV_CONTAMINATION);

    nuv_path.write_to_dat("full_nuv","All effects NUV");

    fuv_path.apply_spectral_response(&FUV_CONTAMINATION);
    fuv_path.write_to_dat("full_fuv","All effects FUV");



    println!("fuv: {:?}",fuv_path.total_average_photon_flux(4417.864669110647)*0.76153773);
    println!("nuv: {:?}",nuv_path.total_average_photon_flux(4417.864669110647)*0.76153773);



}