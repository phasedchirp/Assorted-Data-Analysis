// extern crate statrs;

// use statrs::generate::*;

use std::f64::consts::PI;

struct Sinusoid {
    freq: f64,
    amp: f64,
    offset: f64
}

impl Sinusoid {
    pub fn new(f: f64, a: f64, o: f64) -> Sinusoid {
        Sinusoid{freq: f, amp: a, offset: o}
    }

    pub fn period(&self) -> f64 {
        1.0/self.freq
    }

    pub fn evaluate(&self,ts: &Vec<f64>) -> Vec<f64> {
        let phases = ts.iter().map(|t| 2.0*PI*self.freq*t+self.offset);
        phases.map(|p| self.amp * p.sin()).collect::<Vec<f64>>()
    }

}

fn main() {
    let signal = Sinusoid::new(1.0, 2.0, 0.0);
    let times = vec![0.0,0.25,0.5,0.75,1.0];
    println!("{:?}",signal.evaluate(&times));
}
