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
		compute_one_step(af,&res, &mut new_scores);
		if stabilisation(&res,&new_scores) {
			has_changed = false;
		}
        std::mem::swap(&mut res, &mut new_scores);
	}
	res
}

fn compute_one_step(af : &ArgumentationFramework, scores_arg : &Vec<f64>, res : &mut Vec<f64>) {//-> (Vec<f64>, bool) {
    //let mut res = vec![0.;scoresArg.len()];
    //let mut res = Vec::with_capacity(scoresArg.len());
    //let mut haschanged = true;
	for i in 0..scores_arg.len() {
		let mut sum_score_attacker = 0.;
		for  attacker in &af.af_attacker[i] {
            unsafe {
	            if sum_score_attacker < *scores_arg.get_unchecked(*attacker as usize) {
                    sum_score_attacker = *scores_arg.get_unchecked(*attacker as usize);
                }
            }
		}
		res[i] =  1. / (1. + sum_score_attacker);
	}
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