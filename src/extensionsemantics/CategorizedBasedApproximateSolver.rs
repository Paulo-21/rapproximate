use std::{time::Instant, process::exit};
use crate::{cli::{Heuristic, Problem, Task}, extensionsemantics::{SimpleGroundedSemanticsSolver, SimpleGroundedSemanticsSolver2}, gradualsemantics::{card_based, categorizer::{self, compute_final_score}, counting, max_based, no_self_att_hcat, perso}, graph::ArgumentationFramework};
use crate::cli::Semantics::*;

pub fn solve(mut af : ArgumentationFramework, task : Task) -> bool{
    
	/*Grounded Part */
    let start = Instant::now();
    let mut t = task.clone();
    t.problem = Problem::SE;
	
	let grounded_extension = if task.new {
		SimpleGroundedSemanticsSolver2::solve(&mut af, &t)
	}
	else {
		SimpleGroundedSemanticsSolver::solve(&t, &mut af)
	};
    //let groundedExtension = SimpleGroundedSemanticsSolver2::solve_with_bitset(&mut af, &t);
    if task.verbose {
		print!("{};", start.elapsed().as_millis() as f32/1000.0);
	}
    if grounded_extension.contains(&task.argument) {
		if task.verbose {
			print!("None;None;");
		}	
		return true;
	}
	for  attacker in &af.af_attacker[task.argument] {
		if grounded_extension.contains(&(*attacker as usize)) {
			if task.verbose {
				print!("None;None;");
			}
			return false;
		}
	}
	match task.algo {
		Heuristic::HARPER => {
			match task.problem {
				Problem::DC => true,
				Problem::DS => false,
				_ => panic!("Problem type is not covered")
			}
		},
		Heuristic::HCAT => { /*h-Categorized Part */
			let start = Instant::now();
			let degree = if task.new {
				categorizer::solve_new(af, &task)
			}
			else {
				categorizer::solve(af, &task)
			};
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree >= threshold
		},
		Heuristic::INOUT => { /*Inout Part */
			let threshold = 
			if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			let in_degree = af.inDegree(task.argument);
			let out_degree = af.outDegree(task.argument);
			out_degree >= threshold as usize * in_degree
		},
		Heuristic::NoSelfAtt => {
			let degree = no_self_att_hcat::solve(af, &task);
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree >= threshold
		},
		Heuristic::Card => {
			let degree = card_based::solve(af, &task);
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree >= threshold
		},
		Heuristic::Max => {
			let degree = max_based::solve(af, &task);
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree >= threshold
		},
		Heuristic::Counting => {
			//let degree = counting::solve(af, &task, 3, 0.9);
			let degree = counting::solve2(af, &task, 3, 0.9);
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree >= threshold
		},
		Heuristic::Perso => {
			let hcat = compute_final_score(&af);
			let mut gr = vec![0.5; af.nb_argument];
			for l in grounded_extension { gr[l] = 1.;
				for k in &af.af_attackee[l] { gr[*k as usize] = 0.; }
			}
			let degree = perso::solve(&af, &gr, &hcat);
			if task.verbose {
				print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("{:.17};", degree[task.argument]);
			}
			let threshold = if let Some(t) = task.threshold { t }
			else { choice_threshold(&task) };
			degree[task.argument] >= threshold
		},
	}

}
fn choice_threshold(task : &Task) -> f64 {
    if task.problem == Problem::DC  {
		match task.semantics {
				CO |
				ST|
				SST => {
				    return 0.5
                },
				STG => {
					return 0.
                },
				ID => return 1.,
				_ => {
                    eprintln!("This combination (semantics, problem) is not handled by this solver.");
					exit(1);
                }	
			}
		}
		
		if task.problem == Problem::DS {
			match task.semantics {
				PR |
				SST |
				STG => return 1.,
				ST => return 0.,
				_ => {
					eprintln!("This combination (semantics, problem) is not handled by this solver.");
					exit(1);
                }
			}
        }
        eprintln!("no problem set");
        exit(1);
}