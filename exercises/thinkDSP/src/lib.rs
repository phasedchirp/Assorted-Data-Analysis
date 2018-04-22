use std::f64::consts::{PI};
use std::f64::EPSILON;
use std::ops::{Add, Sub};



// Sinc function (maybe a bit unstable):
pub fn sinc(x: f64) -> f64 {
    if x.abs() <  EPSILON.sqrt() {
        1.0
    } else {
        (PI*x).sin()/(PI*x)
    }
}

pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}


pub trait PeriodicSignal {
    // fn period(&self) -> f64; // Need to work out good general implementation
    fn evaluate(&self, ts: &Vec<f64>) -> Vec<f64>;
}



// trying an alternative approach:
#[derive(Clone)]
pub struct Periodic {
    freq: Vec<f64>,
    amp: Vec<f64>,
    offset: Vec<f64>,
    funcs: Vec<fn(p: f64) -> f64>
}

impl Periodic {
    // make simple signals with less typing:
    pub fn simple(freq: f64, amp: f64, offset: f64, f: fn(p: f64) -> f64) -> Periodic {
        Periodic{freq: vec![freq], amp: vec![amp], offset: vec![offset], funcs: vec![f]}
    }

    pub fn new(freqs: Vec<f64>, amps: Vec<f64>, offsets: Vec<f64>, fs: Vec<fn(p: f64) -> f64>) -> Periodic {
        Periodic{freq: freqs, amp: amps, offset: offsets, funcs: fs}
    }

    fn eval_pt(&self, t: f64) -> f64 {
        let mut ft = 0.0;
        for i in 0..self.freq.len(){
            ft += self.amp[i]*self.funcs[i](2.0*PI*self.freq[i]*t+self.offset[i]);
        }
        ft
    }
}

impl PeriodicSignal for Periodic {
    fn evaluate(&self, ts: &Vec<f64>) -> Vec<f64> {
        ts.iter().map(|t| self.eval_pt(*t)).collect()
    }
}

impl Add for Periodic {
    type Output = Periodic;

    fn add(self, other: Periodic) -> Periodic {
        let mut output = self.clone();
        output.freq.append(&mut other.freq.clone());
        output.amp.append(&mut other.amp.clone());
        output.offset.append(&mut other.offset.clone());
        output.funcs.append(&mut other.funcs.clone());
        output
    }
}

impl Sub for Periodic {
    type Output = Periodic;

    fn sub(self, other: Periodic) -> Periodic {
        let mut output = self.clone();
        output.freq.append(&mut other.freq.clone());
        output.amp.append(&mut other.amp.clone().iter().map(|s| -s).collect());
        output.offset.append(&mut other.offset.clone());
        output.funcs.append(&mut other.funcs.clone());
        output
    }
}




// // Sum of Sine waves:
// pub struct SumSines {
//     components: Vec<Sine>
// }
//
// impl SumSines {
//     pub fn new(s: &Vec<Sine>) -> SumSines {
//         SumSines{components: s.clone()}
//     }
// }

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
