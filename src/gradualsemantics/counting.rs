use std::ops::{Div, Mul};
use crate::graph::ArgumentationFramework;
use crate::cli::Task;
use nalgebra::{DMatrix, Matrix, VecStorage, Dyn};

pub fn solve(af : ArgumentationFramework, task : &Task, k : i32, alpha : f64) -> f64 {
    let mut v = vec![0.;af.nb_argument*af.nb_argument];
    for (i, a) in af.af_attacker.iter().enumerate() {
        for n in a {
            v[af.nb_argument*i + *n as usize] = 1.;
        }
    }
    let mut matrice = DMatrix::from_iterator(af.nb_argument, af.nb_argument, v);
    let mut n = 0.;
    for col in matrice.column_iter() {
        let c : f64 = col.iter().sum();
        if c > n { n = c }
    }
    matrice = matrice.transpose();
    let matrice1 = matrice.div(n);
    let identity = DMatrix::from_column_slice(af.nb_argument, 1, &vec![1.0; af.nb_argument]);
    
    let mut w =  DMatrix::zeros(af.nb_argument, 1) ;
    
    for i in 0..=k {
        w +=  ((-1f64).powi(i) * alpha.powi(i)) * &matrice1.pow(i as u32).mul(&identity) ;
        //print_matrice(&w, af.nb_argument);
    }
    
    *(w.get(task.argument).unwrap())
}

fn print_matrice(matrice : &Matrix<f64, Dyn, Dyn, VecStorage<f64,Dyn,Dyn>>, len : usize) {
    for (i, m) in matrice.iter().enumerate() {
        if i%len == 0 {
            println!();
        }
        print!(" {:.4}", m);
    }
    println!();
}