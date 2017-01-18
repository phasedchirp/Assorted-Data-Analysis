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

// not wrapping the whole sim step in this so that each perturbed data set is seen by both versions.
fn simulate(p:&Vector<f64>, d: &Matrix<f64>, t: &Vector<f64>, res: &mut Vec<f64>,m: &str) {
    let mut lin_mod = LinRegressor::default();
    if m == "default" {
        lin_mod.train(d,t).unwrap();
    } else  if m == "qr" {
        lin_mod.train_with_qr(d,t).unwrap();
    }

    let params = lin_mod.parameters().unwrap();
    res.push((params - p).norm());
}

// fn simulate_pred(p:&Vector<f64>, d: &Matrix<f64>, t: &Vector<f64>, res: &mut Vec<f64>,m: &str) {
//     let mut lin_mod = LinRegressor::default();
//     if m == "default" {
//         lin_mod.train(d,t).unwrap();
//     } else  if m == "qr" {
//         lin_mod.train_with_qr(d,t).unwrap();
//     }
//
//     let preds = lin_mod.predict(&d).unwrap();
//     res.push((preds - p).norm());
// }



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

    let mut results_def = Vec::new();
    let mut results_def_s = Vec::new();
    let mut results_def_c = Vec::new();

    let mut results_qr = Vec::new();
    let mut results_qr_s = Vec::new();
    let mut results_qr_c = Vec::new();

    for _ in 0..n {
        // noise uniform between -lim and lim
        let lim = 0.0001;
        let perturbed = perturb_matrix(&data,&mut rng,lim);
        let perturbed_s = perturb_matrix(&data_s,&mut rng,lim);
        let perturbed_c = perturb_matrix(&data_c,&mut rng,lim);

        simulate(&params_ref,&perturbed,&target,&mut results_def,"default");
        simulate(&params_ref_qr,&perturbed,&target,&mut results_qr,"qr");

        simulate(&params_ref_s,&perturbed_s,&target_s,&mut results_def_s,"default");
        simulate(&params_ref_qr_s,&perturbed_s,&target_s,&mut results_qr_s,"qr");

        simulate(&params_ref_c,&perturbed_c,&target_c,&mut results_def_c,"default");
        simulate(&params_ref_qr_c,&perturbed_c,&target_c,&mut results_qr_c,"qr");
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
