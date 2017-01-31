// extern crate statrs;

// use statrs::generate::*;



mod lib;
use lib::*;

fn main() {
    let signal_1 = Periodic::simple(1.0, 2.0, 0.0,sin);
    let signal_2 = Periodic::simple(1.0, 2.0, 0.0,sinc);
    let signal_3 = Periodic::new(vec![1.0,1.0],vec![2.0,2.0],vec![0.0,0.0],vec![sinc,sin]);

    let times = vec![0.0,0.25,0.5,0.75,1.0];
    println!("{:?}",signal_1.evaluate(&times));
    println!("{:?}",signal_2.evaluate(&times));
    let sum_of_signals = signal_1 + signal_2;
    println!("{:?}", sum_of_signals.evaluate(&times));
    println!("{:?}",signal_3.evaluate(&times));
}
