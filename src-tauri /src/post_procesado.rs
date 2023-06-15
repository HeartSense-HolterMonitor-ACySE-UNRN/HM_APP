use realfft::RealFftPlanner;
use rustfft::{num_complex::Complex};
use filters::filter::Filter;
use std::f64::consts::PI;

pub const TIEMPOENTREMUESTRAS: f64 = 0.1; 
const F_MIN: f64 = 2.0;
const F_MAX: f64 = 32.0;


pub fn post_process(mut ret: &mut Vec<f64>){
    let lenght = ret.len();
    let mut planner = RealFftPlanner::<f64>::new();

    let fft = planner.plan_fft_forward(lenght);
    let mut spectrum = fft.make_output_vec();

    let ffti = planner.plan_fft_inverse(lenght);

    fft.process(&mut ret, &mut spectrum).unwrap();

    let post_process_filter = (|&f: &f64| f<F_MIN).or(|&f: &f64| f>F_MAX);
    let delta_f = (2.0f64*PI)/(TIEMPOENTREMUESTRAS*spectrum.len() as f64);
    spectrum = spectrum
                .iter()
                .enumerate()
                .map(|(f, &a)| {
                    let f = f as f64 * delta_f;
                    if post_process_filter.filter(&f){
                        Complex{ re: 0.0f64, im: 0.0f64}
                    }else{a} 
                }).collect();
    
    ffti.process(&mut spectrum, &mut ret).unwrap();
    *ret = ret.iter().map(|&c| c/(lenght as f64)).collect();
}
