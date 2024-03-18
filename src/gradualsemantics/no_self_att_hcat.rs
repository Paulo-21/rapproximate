use crate::{graph::ArgumentationFramework, cli::Task};

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task : &Task) -> f64 {
    let score = compute_final_score(&af);
    score[task.argument]
}

fn compute_final_score(af : &ArgumentationFramework) -> Vec<f64> {
    let mut res = init_scores(af);
    let mut new_scores = init_scores(af);
    let mut has_changed = true;
    
		while has_changed {
			has_changed = false;
			for i in 0..res.len() {
				let mut sum_score_attacker = 0.;
				let ic = i as i32;
				for  attacker in &af.af_attacker[i] {
					if ic == *attacker {
						new_scores[i] = 0.;
						break;
					}
					unsafe {
						sum_score_attacker += res.get_unchecked(*attacker as usize);
					}
				}
				if new_scores[i] == 0. { continue; }

				new_scores[i] =  1. / (1. + sum_score_attacker);
				if (new_scores[i] - res[i]).abs() > EPSILON {
					has_changed = true;
				}
			}
            std::mem::swap(&mut res, &mut new_scores);
		}
		res
}

fn init_scores(af : &ArgumentationFramework) -> Vec<f64> {
    vec![1.0;af.nb_argument]
}

fn stabilisation(tab1 : &[f64], tab2 : &[f64]) -> bool {
	for (i, x) in tab1.iter().enumerate() {
		if (x-tab2[i]).abs() > EPSILON {
			return false;
		}
	}
	true
}