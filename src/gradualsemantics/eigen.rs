use std::{process::exit, time::Instant};

use crate::{graph::ArgumentationFramework, cli::Task};


pub fn solve(af : ArgumentationFramework, _task : &Task) -> Vec<f64> {
    let start = Instant::now();
    let score = compute_final_score(&af);
    println!("Computed in {} ms", start.elapsed().as_millis());
    println!("{}", score.last().unwrap());
    score
}
#[allow(unused_assignments)]
fn compute_final_score(af : &ArgumentationFramework) -> Vec<f64> {
    let mut last = init_scores(af);
    let mut new = init_scores(af);
    let max_iter = 100;
    let tol = 0.00001;
    for _ in 0..max_iter {
        //let x_last = x.clone();
        last = new;
        new = last.clone();
        for node_index in 0..af.nb_argument {
            for target in &af.af_attackee[node_index] {
                new[*target as usize] += last[node_index];
            }
        }
        let norm: f64 = new.iter().map(|val| val.powi(2)).sum::<f64>().sqrt();
        if norm == 0. {
            println!("ZERO");
            exit(1);
        }
        for v in new.iter_mut() {
            *v /= norm;
        }
        if (0..new.len())
            .map(|node| (new[node] - last[node]).abs())
            .sum::<f64>()
            < af.nb_argument as f64 * tol
        {
            return new;
        }
    }

	new
}

fn init_scores(af : &ArgumentationFramework) -> Vec<f64> {
    vec![1.0;af.nb_argument]
}
