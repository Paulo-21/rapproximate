use crate::{graph::ArgumentationFramework, cli::Task};

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task : &Task) -> f64 {
    let score = computeFinalScore(&af);
    score[task.argument]
}

fn computeFinalScore(af : &ArgumentationFramework) -> Vec<f64> {
    let mut res = initScores(af);
    let mut newScores = initScores(af);
    let mut hasChanged = true;
    
		while hasChanged {
			/*(newScores, hasChanged) =*/ computeOneStep(af,&res, &mut newScores);
			if stabilisation(&res,&newScores) {
				hasChanged = false;
			}
            std::mem::swap(&mut res, &mut newScores);
		}
		res
}

fn computeOneStep(af : &ArgumentationFramework, scoresArg : &Vec<f64>, res : &mut Vec<f64>) {//-> (Vec<f64>, bool) {
    //let mut res = vec![0.;scoresArg.len()];
    //let mut res = Vec::with_capacity(scoresArg.len());
    //let mut haschanged = true;
		for i in 0..scoresArg.len() {
			let mut sumScoreAttacker = 0.;
            if af.af_attacker[i].contains(&(i as i32)) {
                res[i] = 0.;
                continue;
            }
			for  attacker in &af.af_attacker[i] {
                unsafe {
                    sumScoreAttacker += scoresArg.get_unchecked(*attacker as usize);
                }
			}
			res[i] =  1. / (1. + sumScoreAttacker);
		}
		//return (res, haschanged);
}
fn initScores(af : &ArgumentationFramework) -> Vec<f64> {
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