// extern crate statrs;

// use statrs::generate::*;



mod lib;
use lib::*;

fn main() {
    let signal_1 = Sine::new(1.0, 2.0, 0.0);
    let signal_2 = Sinc::new(1.0, 2.0, 0.0);

    let times = vec![0.0,0.25,0.5,0.75,1.0];
    println!("{:?}",signal_1.evaluate(&times));
    println!("{:?}",signal_2.evaluate(&times));
}
