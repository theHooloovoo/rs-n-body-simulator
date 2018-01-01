// Written by theHooloovoo

extern crate getopts;
extern crate rayon;
extern crate rand;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod body;
use body::Body;

fn main() {
    let com_args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    // Delcare options and flags for getopts
    opts.optopt("n", "", "Number of iterations. Must be a natural number.", "NUMBER");
    opts.optopt("t", "", "Length of timestep (delta-time). Must be a positve real number.", "DT");
    opts.optopt("f", "file", "Input file containing bodies to simulate.", "FILE");
    opts.optopt("o", "output", "Output file containing record of simulation", "OUTPUT");


    // Variables used to control the simulation
    let mut body_vec: Vec<body::Body> = Vec::new();
    let steps: i32; // Assigned by the -n flag
    let dt: f64;    // Assigned by the -t flag
    let f_in:  String;  // Assigned by the -f flag
    let f_out: String;  // Assigned by the -o flag

    // Used to keep track of world state over time
    let mut journal: Vec<Vec<Body>> = Vec::new();

    // Collect matches from command line
    let matches = match opts.parse(&com_args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Assign the steps variable
    match matches.opt_str("n") {
        Some(n) => steps = n.parse::<i32>().unwrap(),
        None    => steps = 1,
    };
    // Assign the dt variable
    match matches.opt_str("t") {
        Some(n) => dt = n.parse::<f64>().unwrap(),
        None    => dt = 1.0,
    };
    // Assign the f_in variable
    match matches.opt_str("f") {
        Some(f) => f_in = f.clone(),
        None    => f_in = "".to_string(),
    };


    // Open f_in and populate body_vec
    let mut f_contents = String::new();
    let mut f = File::open(&f_in).expect("Oops, couldn't open file!");
    f.read_to_string(&mut f_contents).expect("Oops, couldn't read file!");
    for n in f_contents.lines() {
        match Body::from_string(&n) {
            Some(b) => body_vec.push(b),
            None    => {},
        }
    }

    println!("steps = {}", steps);
    println!("dt = {}", dt);
    println!("Bodies = {}", body_vec.len());

    for _ in 0..steps {
        Body::iterate(&mut body_vec, dt );
        journal.push(body_vec.clone());
        /*
        for n in 0..body_vec.len() {
            // body_vec[n].print();
            println!("{}", body_vec[n].to_string());
        }
        */
    }
}
