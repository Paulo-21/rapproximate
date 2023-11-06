use std::{process::exit};

use crate::{parser::{Problem, Task}, graph::ArgumentationFramework};
#[derive(Clone, PartialEq)]
pub enum Label {
    IN, OUT, UNDEC
}

pub fn solve(task : &Task, af : &mut ArgumentationFramework) -> Vec<usize> {
    let problem_type = task.problem;
    let mut labelling = initLabelling(&af);
    
    let mut hasChanged = true;
    while hasChanged {
        let new_labelling = propagateDefense(&af, &labelling);
        hasChanged = !sameLabelling(&new_labelling, &labelling);
        /*if sameLabelling(&new_labelling, &labelling) {
            hasChanged = false;
        }*/
        labelling = new_labelling;
    }
    let mut solution = Vec::<usize>::with_capacity(labelling.len());
    match problem_type {
        Problem::SE => {
            for (arg, label) in labelling.into_iter().enumerate() {
                match label {
                    Label::IN => {
                        solution.push(arg);
                    }
                    _=> {}
                }
            }
            return solution;
        },/*Je n'ai pas l'impression que ce type de problème ne soit réellement traité.
        Problem::DC | Problem::DS => {
            match labelling[argument] {
                Label::IN => {
                    true
                },
                _=> { false }
            }
        }*/
        _=>{
            exit(1);
        }
    }
}

fn initLabelling( af : &ArgumentationFramework) -> Vec<Label> {
    let mut labelling : Vec<Label> = Vec::with_capacity(af.nb_argument);
	for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			labelling.push(Label::IN);
		} else {
		    labelling.push(Label::UNDEC);
		}
	}
	return labelling;
}

fn propagateDefense(af : &ArgumentationFramework, labelling : &Vec<Label>) -> Vec<Label> {
    let mut result =  vec![Label::UNDEC;labelling.len()];

		// We check all the arguments of the AF and if an argument has the label IN then all the arguments it attacks have the label OUT.
		for i in 0..labelling.len() {
			match labelling[i] {
                Label::IN => {
                    result[i] = Label::IN;
                    for argument in &af.af_attackee[i] {
                        result[(*argument) as usize] = Label::OUT;
                    }
                },
                _=>{}
			}
		}

		for i in 0..labelling.len() {
			if result[i] == Label::UNDEC && allAttackersAreOut(af, labelling, i) {
				result[i] = Label::IN;
			}
		}
		return result;
}
fn allAttackersAreOut(af : &ArgumentationFramework, labelling : &Vec<Label>, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if labelling[(*attacker) as usize] != Label::OUT {
			return false;
		}
	}
	true
}
fn sameLabelling(labelling1 : &Vec<Label>, labelling2 : &Vec<Label>) -> bool {
    if labelling1.len() != labelling2.len() {
	    return false;
	}
	for i in 0..labelling1.len() {
		if labelling1[i] != labelling2[i] {
			return false;
		}
	}
    true
}