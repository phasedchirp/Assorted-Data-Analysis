#[macro_use] extern crate rusty_machine;
extern crate rand;

use rusty_machine::linalg::{Matrix,BaseMatrix,Vector,Metric};
use rusty_machine::learning::SupModel;
use rusty_machine::learning::lin_reg::LinRegressor;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

mod dataset;
use dataset::*;

// fn perturb_vector<R: Rng>(v: &Vector<f64>, r: &mut R, lim: f64) -> Vector<f64> {
//     let noise = Range::new(-lim, lim);
//     let vals: Vec<f64> = v.data().iter().map(|x| *x + noise.ind_sample(r)).collect();
//     Vector::new(vals)
// }

fn perturb_matrix<R: Rng>(a: &Matrix<f64>, r: &mut R, lim: f64) -> Matrix<f64> {
    let noise = Range::new(-lim, lim);
    let vals: Vec<f64> = a.data().iter().map(|x| *x + noise.ind_sample(r)).collect();
    Matrix::new(a.rows(),a.cols(),vals)
}



fn main() {
    let n = 1000;
    let mut rng = rand::thread_rng();
    let (data,target) = generate_data();
    let (data_s,target_s) = near_singular_data();
    let (data_c,target_c) = collinear_data();


    let mut lin_mod = LinRegressor::default();
    lin_mod.train(&data,&target).unwrap();
    let params_ref = lin_mod.parameters().unwrap().to_owned();

    lin_mod.train(&data_s,&target_s).unwrap();
    let params_ref_s = lin_mod.parameters().unwrap().to_owned();

    lin_mod.train(&data_c,&target_c).unwrap();
    let params_ref_c = lin_mod.parameters().unwrap().to_owned();

    lin_mod.train_with_qr(&data,&target).unwrap();
    let params_ref_qr = lin_mod.parameters().unwrap().to_owned();

    lin_mod.train_with_qr(&data_s,&target_s).unwrap();
    let params_ref_qr_s = lin_mod.parameters().unwrap().to_owned();

    lin_mod.train_with_qr(&data_c,&target_c).unwrap();
    let params_ref_qr_c = lin_mod.parameters().unwrap().to_owned();


    // repeatedly fit models on perturbed data to examine stability
    // uniform between -lim and lim
    let mut results_def = Vec::new();
    let mut results_def_s = Vec::new();
    let mut results_def_c = Vec::new();

    let mut results_qr = Vec::new();
    let mut results_qr_s = Vec::new();
    let mut results_qr_c = Vec::new();
    for _ in 0..n {
        let perturbed = perturb_matrix(&data,&mut rng,0.01);
        let perturbed_s = perturb_matrix(&data_s,&mut rng,0.01);
        let perturbed_c = perturb_matrix(&data_c,&mut rng,0.01);

        lin_mod.train(&perturbed,&target).unwrap();
        let params_temp = lin_mod.parameters().unwrap().to_owned();
        results_def.push((params_temp - &params_ref).norm());
        lin_mod.train_with_qr(&perturbed,&target).unwrap();
        let params_temp_qr = lin_mod.parameters().unwrap().to_owned();
        results_qr.push((params_temp_qr - &params_ref_qr).norm());

        lin_mod.train(&perturbed_s,&target_s).unwrap();
        let params_temp_s = lin_mod.parameters().unwrap().to_owned();
        results_def_s.push((params_temp_s - &params_ref_s).norm());
        lin_mod.train_with_qr(&perturbed_s,&target_s).unwrap();
        let params_temp_qr_s = lin_mod.parameters().unwrap().to_owned();
        results_qr_s.push((params_temp_qr_s - &params_ref_qr_s).norm());

        lin_mod.train(&perturbed_c,&target_c).unwrap();
        let params_temp_c = lin_mod.parameters().unwrap().to_owned();
        results_def_c.push((params_temp_c - &params_ref_c).norm());
        lin_mod.train_with_qr(&perturbed_c,&target_c).unwrap();
        let params_temp_qr_c = lin_mod.parameters().unwrap().to_owned();
        results_qr_c.push((params_temp_qr_c - &params_ref_qr_c).norm());
    }

    let def = Vector::new(results_def);
    let qr = Vector::new(results_qr);
    let def_s = Vector::new(results_def_s);
    let qr_s = Vector::new(results_qr_s);
    let def_c = Vector::new(results_def_c);
    let qr_c = Vector::new(results_qr_c);

    println!("Performance on realistic data:");
    println!("Average diff for default method:       {}",def.mean());
    println!("Standard Deviation for default method: {}",def.variance().sqrt());
    println!("Average diff for QR method:            {}",qr.mean());
    println!("Standard Deviation for QR method:      {}\n",qr.variance().sqrt());

    println!("Performance when matrix is nearly singular:");
    println!("Average diff for default method:       {}",def_s.mean());
    println!("Standard Deviation for default method: {}",def_s.variance().sqrt());
    println!("Average diff for QR method:            {}",qr_s.mean());
    println!("Standard Deviation for QR method:      {}\n",qr_s.variance().sqrt());

    println!("Slightly more realistic data with collinearity problems:");
    println!("Average diff for default method:       {}",def_c.mean());
    println!("Standard Deviation for default method: {}",def_c.variance().sqrt());
    println!("Average diff for QR method:            {}",qr_c.mean());
    println!("Standard Deviation for QR method:      {}",qr_c.variance().sqrt());
}
