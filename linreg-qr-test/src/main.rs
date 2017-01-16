#[macro_use] extern crate rusty_machine;
extern crate rand;

use rusty_machine::linalg::{Matrix,BaseMatrix,Vector,Metric};
use rusty_machine::learning::SupModel;
use rusty_machine::learning::lin_reg::LinRegressor;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

mod dataset;
use dataset::*;

fn perturb_vector<R: Rng>(v: &Vector<f64>, r: &mut R, lim: f64) -> Vector<f64> {
    let noise = Range::new(-lim, lim);
    let vals: Vec<f64> = v.data().iter().map(|mut x| *x + noise.ind_sample(r)).collect();
    Vector::new(vals)
}

fn perturb_matrix<R: Rng>(a: &Matrix<f64>, r: &mut R, lim: f64) -> Matrix<f64> {
    let noise = Range::new(-lim, lim);
    let vals: Vec<f64> = a.data().iter().map(|mut x| *x + noise.ind_sample(r)).collect();
    Matrix::new(a.rows(),a.cols(),vals)
}



fn main() {
    let n = 1000;
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
    let mut results_def = Vec::new();
    let mut results_qr = Vec::new();
    for _ in 0..n {
        // println!("{:?}", perturb_vector(&target,&mut rng,1.0).select(&[1,2,3]));
        let perturbed = perturb_matrix(&data,&mut rng,1.0);
        lin_mod.train(&perturbed,&target).unwrap();
        let params_temp = lin_mod.parameters().unwrap().to_owned();
        results_def.push((params_temp - &params_ref).norm());
        lin_mod.train_with_qr(&perturbed,&target).unwrap();
        let params_temp_qr = lin_mod.parameters().unwrap().to_owned();
        results_qr.push((params_temp_qr - &params_ref_qr).norm());
    }
    let ave_def = results_def.iter().fold(0.0, |a, x| a + x) /(n as f64) ;
    let ave_qr = results_qr.iter().fold(0.0, |a, x| a + x) /(n as f64) ;
    println!("Average diff for default method: {}",ave_def);
    println!("Average diff for QR method:      {}",ave_qr);
}
