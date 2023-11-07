use std::{time::Instant, process::exit};
use crate::{graph::ArgumentationFramework, cli::{Problem, Task}, extensionsemantics::{SimpleGroundedSemanticsSolver, SimpleGroundedSemanticsSolver2}, gradualsemantics::categorizer};
use crate::cli::Semantics::*;

pub fn solve(mut af : ArgumentationFramework, mut task : Task) -> bool{
    
    /*Grounded Part */
    let start = Instant::now();
    let mut t = task.clone();
    t.problem = Problem::SE;
	
    let groundedExtension = SimpleGroundedSemanticsSolver::solve(&t, &mut af);
    //let groundedExtension = SimpleGroundedSemanticsSolver2::solve(&mut af, &t);
    print!("{};", start.elapsed().as_millis() as f32/1000.0);
    if groundedExtension.contains(&task.argument) {
		print!("None;None;");
		return true;
	}
	for  attacker in &af.af_attacker[task.argument] {
		if groundedExtension.contains(&(*attacker as usize)) {
		    print!("None;None;");
			return false;
		}
	}
    /*h-Categorized Part */
    let start = Instant::now();
    let degree = categorizer::solve(af, &task);
    print!("{};", start.elapsed().as_millis() as f32 / 1000.);
    print!("{:.17};", degree);
    let threshold = choice_threshold(&task);
	if degree >= threshold {
		return true;
	} else {
		return false;
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