#[macro_use] extern crate rusty_machine;
// #[macro_use] extern crate rulinalg;
extern crate rand;

// use rulinalg::matrix::Matrix;
// use rulinalg::vector::Vector;

use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;
use rusty_machine::learning::SupModel;
use rusty_machine::learning::lin_reg::LinRegressor;
// use libnum::abs;
// use test::{Bencher, black_box};
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

fn generate_data() -> (Matrix<f64>, Vector<f64>) {
    // Training data
    let data = Matrix::new(31,2,vec![8.3, 70., 8.6, 65., 8.8, 63., 10.5, 72.,
                       10.7, 81., 10.8, 83., 11.0, 66., 11.0, 75.,
                       11.1, 80., 11.2, 75., 11.3, 79., 11.4, 76.,
                       11.4, 76., 11.7, 69., 12.0, 75., 12.9, 74.,
                       12.9, 85., 13.3, 86., 13.7, 71., 13.8, 64.,
                       14.0, 78., 14.2, 80., 14.5, 74., 16.0, 72.,
                       16.3, 77., 17.3, 81., 17.5, 82., 17.9, 80.,
                       18.0, 80., 18.0, 80., 20.6, 87.]);
    let target = Vector::new(vec![10.3, 10.3, 10.2, 16.4, 18.8, 19.7, 15.6, 18.2, 22.6, 19.9, 24.2, 21.0, 21.4, 21.3, 19.1, 22.2, 33.8, 27.4, 25.7, 24.9, 34.5, 31.7, 36.3, 38.3, 42.6, 55.4, 55.7, 58.3, 51.5, 51.0, 77.0]);

    (data,target)
}

fn perturb_vector<R: Rng>(v: &Vector<f64>, r: &mut R, lim: f64) -> Vector<f64> {
    let mut u = v.clone();
    let noise = Range::new(-lim, lim);
    for x in u.mut_data() {
        *x += noise.ind_sample(r);
    }
    u
}



fn main() {
    let mut rng = rand::thread_rng();
    let (data,target) = generate_data();
    // let test = perturb_vector(target.clone(),&mut rng, 0.5);
    let mut lin_mod = LinRegressor::default();
    lin_mod.train(&data,&target).unwrap();
    let params_ref = lin_mod.parameters().unwrap().to_owned();
    lin_mod.train_with_qr(&data,&target).unwrap();
    let params_ref_qr = lin_mod.parameters().unwrap().to_owned();
    // repeatedly fit models on perturbed data to examine stability
    // uniform between -lim and lim
    for _ in 0..5 {
        let perturbed = perturb_vector(&target,&mut rng,0.5);
        lin_mod.train(&data,&perturbed).unwrap();
        let params_temp = lin_mod.parameters().unwrap().to_owned();
        println!("{:?}",params_temp - &params_ref);
        lin_mod.train_with_qr(&data,&perturbed).unwrap();
        let params_temp_qr = lin_mod.parameters().unwrap().to_owned();
        println!("{:?}",params_temp_qr - &params_ref_qr);
    }
}
