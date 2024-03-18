use crate::{graph::ArgumentationFramework, cli::Task};

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task : &Task) -> f64 {
    let score = compute_final_score(&af);
    
	//let solution= computeFinalScore2(&af, task.argument);
	//let solution= computeFinalScore2_test(&af, task.argument); // BEST
	//let solution= compute_final_score2_deep(&af, task.argument);
    score[task.argument]
}
pub fn solve_new(af : ArgumentationFramework, task : &Task) -> f64 {
	compute_final_score2_test(&af, task.argument) // BEST
}

fn compute_final_score(af : &ArgumentationFramework) -> Vec<f64> {
    let mut res = init_scores(af);
    let mut new_scores = init_scores(af);
    let mut has_changed = true;
		while has_changed {
			has_changed = false;
			for i in 0..res.len() {
				let mut sum_score_attacker = 0.;
				for  attacker in &af.af_attacker[i] {
					unsafe {
						sum_score_attacker += res.get_unchecked(*attacker as usize);
					}
				}
				new_scores[i] =  1. / (1. + sum_score_attacker);
				if (new_scores[i] - res[i]).abs() > EPSILON {
					has_changed = true;
				}
			}
            std::mem::swap(&mut res, &mut new_scores);
		}
		res
}

fn compute_final_score2( af : &ArgumentationFramework, task_argument : usize) -> f64 {
	let mut nb_hit = 0;
	let mut index_to_hit = Vec::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg : Vec<f64> = vec![1.;af.nb_argument]; 
	index_to_hit.push(task_argument);
	let mut old_score_t_arg = 0.;
	loop  {
		while nb_hit < index_to_hit.len() {
			let arg = index_to_hit[nb_hit];
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[arg] {
				if never_hit[*new_arg as usize] {
					index_to_hit.push(*new_arg as usize);
					never_hit[*new_arg as usize] = false;
				}
				sum_score_attacker += scores_arg[*new_arg as usize];
			}
			scores_arg[arg] = 1. / (1. + sum_score_attacker);
			nb_hit+=1;
		}
		index_to_hit = Vec::with_capacity(af.nb_argument);
		never_hit = vec![true; af.nb_argument];
		nb_hit = 0;
		index_to_hit.push(task_argument);
		if (old_score_t_arg - scores_arg[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg = scores_arg[task_argument];
	}
	old_score_t_arg
}
fn compute_final_score2_test( af : &ArgumentationFramework, task_argument : usize) -> f64 {
	let mut nb_hit = 0;
	let mut index_to_hit = Vec::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg : Vec<f64> = vec![1.;af.nb_argument]; 
	index_to_hit.push(task_argument);
	let mut old_score_t_arg = 0.;
	while nb_hit < index_to_hit.len() {
		let arg = index_to_hit[nb_hit];
		for new_arg in &af.af_attacker[arg] {
			if never_hit[*new_arg as usize] {
				index_to_hit.push(*new_arg as usize);
				never_hit[*new_arg as usize] = false;
			}
		}
		nb_hit+=1;
	}
	loop  {
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				sum_score_attacker += scores_arg[*new_arg as usize];
			}
			scores_arg[*arg] = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg - scores_arg[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg = scores_arg[task_argument];
	}
	old_score_t_arg
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