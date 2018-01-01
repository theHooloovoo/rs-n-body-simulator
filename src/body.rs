extern crate rayon;

use rayon::prelude::*;

// use std::io::Error;

// extern crate rand;
// use rand::{thread_rng, Rng};


const G: f64 = 0.004302;

#[derive(Clone)]
pub struct Body {
    name:   String,
    mass:   f64,
    loc:    [f64; 3],
    vel:    [f64; 3],
}

impl Body {
    // Generate a new Body struct, with full control
    // over all internal data
    pub fn new(l1: f64, l2: f64, l3: f64,
               v1: f64, v2: f64, v3: f64, m: f64, n: &str) -> Body {
        Body {
            loc:    [l1, l2, l3],
            vel:    [v1, v2, v3],
            mass:   m,
            name:   n.to_string(),
        }
    }

    // Used to save contents to a file
    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        
        s.push_str(&self.name.to_string());   s.push_str(" ");
        s.push_str(&self.mass.to_string());   s.push_str(" ");

        s.push_str(&self.loc[0].to_string()); s.push_str(" ");
        s.push_str(&self.loc[1].to_string()); s.push_str(" ");
        s.push_str(&self.loc[2].to_string()); s.push_str(" ");

        s.push_str(&self.vel[0].to_string()); s.push_str(" ");
        s.push_str(&self.vel[1].to_string()); s.push_str(" ");
        s.push_str(&self.vel[2].to_string()); s.push_str(" ");

        s.to_string()
    }

    // Used to recover struct from a text file
    pub fn from_string(s: &str) -> Option<Body> {
        let mut fields = s.split_whitespace().collect::<Vec<&str>>();
        let len = fields.len();

        // Quit if string is of bad form
        if len > 8 || len < 7 { eprintln!("Err: Attempted to parse line into body, but not enough data!\n{}",
                                            s);
                                  return None };
        
        let mut b_name = "".to_string();
        //                mass l_x  l_y  l_z  v_x  v_y  v_z
        let mut values = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        if fields.len() == 8 {
            b_name = fields[0].to_string();
            fields.remove(0);
        };

        for n in 0..fields.len() {
            match fields[n].parse::<f64>() {
                Ok(i)  => values[n] = i,
                Err(text) => { eprintln!("{}", text); // Tell me what went wrong
                               return None },
            }
        }

        Some(Body {
            name:   b_name,
            mass:   values[0],
            loc:    [values[1], values[2], values[3]],
            vel:    [values[4], values[5], values[6]],
        } )
    }

    // Generate a Body with an arbitrary location and mass, but with 
    // zero velocity and acceleration
    pub fn gen_still_body(l1: f64, l2: f64, l3: f64, m: f64) -> Body {
        Body {
            loc:    [l1, l2, l3],
            vel:    [0.0, 0.0, 0.0],
            mass:   m,
            name:   "p".to_string(),
        }
    }

    // Gets the euclidean distance from one 3d point to another
    fn dist(l1: &[f64; 3], l2: &[f64; 3]) -> f64 {
        ( (l1[0] - l2[0]).powi(2) +
          (l1[1] - l2[1]).powi(2) +
          (l1[2] - l2[2]).powi(2) ).sqrt()
    }


    // Parallelalizable ( <- what?)
    fn iterate_velocity(body_v: &mut [f64; 3],   body_l: &[f64; 3],   body_m: &f64,
                            l_list: &Vec<&[f64; 3]>, m_list: &Vec<&f64>, dt: f64) {
        let mut body_f = [0.0, 0.0, 0.0];

        for n in 0..l_list.len() {
            // Find the distance between the given body, and the nth body in the list
            let d = Body::dist(body_l, l_list[n]);

            // If both bodies have the same location, avoid the division by zero
            if d == 0.0 { continue };

            // Calculate the force along the shortest distance between the two
            let f: f64 = ( G * body_m * m_list[n] ) / (d * d);

            // Calculate the force along the standard axis
            body_f[0] += f * ( ( l_list[n][0] - body_l[0] ) / d );
            body_f[1] += f * ( ( l_list[n][1] - body_l[1] ) / d );
            body_f[2] += f * ( ( l_list[n][2] - body_l[2] ) / d );
        }

        // Finally iterate the velocity
        body_v[0] += body_f[0] * dt;
        body_v[1] += body_f[1] * dt;
        body_v[2] += body_f[2] * dt;

    }

    // TODO: Add code to make parallel
    pub fn iterate(bodies: &mut Vec<Body>, dt: f64) {
        { // Temporarily create some vectors to use for iterate_velocity()
            let len = bodies.len();
            // Generate list of locations and velocities
            let mut v_list: Vec<&mut [f64; 3]> = Vec::with_capacity(len);
            let mut l_list: Vec<    &[f64; 3]> = Vec::with_capacity(len);
            let mut m_list: Vec<         &f64> = Vec::with_capacity(len);
            // Populate each list
            for n in bodies.iter_mut() {
                v_list.push(&mut n.vel);
                l_list.push(&n.loc);
                m_list.push(&n.mass);
            }
            for n in 0..len {
                Body::iterate_velocity( &mut v_list[n],
                                        l_list[n], m_list[n],
                                        &l_list,  &m_list, dt );
            }
            // bodies.par_iter_mut().for_each(|n| Body::iterate_velocity(&mut n.vel, &n.loc, &n.mass,
            //                                                           &l_list, &m_list, dt) );
                                                                    
        } // Destroy the old vectors because we don't need them anymore
        
        // Iterate the locations of all the bodies
        for n in bodies.iter_mut() {
            n.loc[0] += n.vel[0] * dt;
            n.loc[1] += n.vel[1] * dt;
            n.loc[2] += n.vel[2] * dt;
        }


    }

    pub fn center_of_mass(v: &Vec<Body>) -> [f64; 3] {
        let mut m = 0.0;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for n in v {
            m += n.mass;
            x += n.mass * n.loc[0];
            y += n.mass * n.loc[1];
            z += n.mass * n.loc[2];
        }

        [x / m, y / m, z / m]
    }

}
