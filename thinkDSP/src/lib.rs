use std::f64::consts::{PI};
use std::f64::EPSILON;



// Sinc function (maybe a bit unstable):
fn sinc(x: f64) -> f64 {
    if x.abs() <  EPSILON.sqrt() {
        1.0
    } else {
        (PI*x).sin()/(PI*x)
    }
}




pub trait Sinusoid {
    fn period(&self) -> f64;
    fn evaluate(&self, ts: &Vec<f64>) -> Vec<f64>;
}


// Sine wave
#[derive(Clone)]
pub struct Sine {
    freq: f64,
    amp: f64,
    offset: f64
}

impl Sine {
    pub fn new(f: f64, a: f64, o: f64) -> Sine {
        Sine{freq: f, amp: a, offset: o}
    }
}

impl Sinusoid for Sine {
    fn period(&self) -> f64 {
        1.0/self.freq
    }

    fn evaluate(&self,ts: &Vec<f64>) -> Vec<f64> {
        let phases = ts.iter().map(|t| 2.0*PI*self.freq*t+self.offset);
        phases.map(|p| self.amp * p.sin()).collect::<Vec<f64>>()
    }

}


// Sinc wave:
#[derive(Clone)]
pub struct Sinc {
    freq: f64,
    amp: f64,
    offset: f64
}

impl Sinc {
    pub fn new(f: f64, a: f64, o: f64) -> Sinc {
        Sinc{freq: f, amp: a, offset: o}
    }
}

impl Sinusoid for Sinc {
    fn period(&self) -> f64 {
        1.0/self.freq
    }

    fn evaluate(&self,ts: &Vec<f64>) -> Vec<f64> {
        let phases = ts.iter().map(|t| 2.0*PI*self.freq*t+self.offset);
        phases.map(|p| self.amp * sinc(p)).collect::<Vec<f64>>()
    }
}

// Sum of Sine waves:
pub struct SumSines {
    components: Vec<Sine>
}

impl SumSines {
    pub fn new(s: &Vec<Sine>) -> SumSines {
        SumSines{components: s.clone()}
    }
}

// impl Sinusoid for SumSines {
//     // this version only accurate for harmonic complexes:
//     fn period(&self) -> f64 {
//         let mut ps: Vec<f64> = Vec::new();
//         for c in self.components {
//             ps.push(c.period());
//         }
//         ps.max()
//         // self.components.map(|s| s.period()).collect::<Vec<f64>>().max()
//     }
//
//     fn evaluate(&self,ts: &Vec<f64>) -> Vec<f64> {
//         let result = Vec::new();
//         let cs: Vec<Vec<f64>> = Vec::new();
//         for c in self.components {
//             cs.push(c.evaluate(ts));
//         }
//         // let vs: Vec<Vec<f64>> = self.components.map(|s| s.evaluate(ts)).collect();
//         for i in 0..ts.len() {
//             let mut val = 0;
//             for j in 0..cs.len(){
//                 val += cs[j][i];
//             }
//             result.push(val);
//         }
//         result
//     }
// }
