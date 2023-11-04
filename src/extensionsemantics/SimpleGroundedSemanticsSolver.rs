use std::process::exit;

use crate::{parser::{Format, Problem, Task}, graph::ArgumentationFramework};

enum Label {
    IN, OUT, UNDEC
}
pub trait Solution {
    fn get_solution(&self) -> bool;
}
pub fn solve(task : Task, af : &mut ArgumentationFramework) -> Vec<usize> {
    let problem_type = task.problem;
    let mut labelling = initLabelling(&af);
    let argument = 1;
    let mut hasChanged = true;
    while hasChanged {
        let new_labelling = propagateDefense(&af, &labelling);
        hasChanged = !sameLabelling(&new_labelling, &labelling);
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
    let labelling : Vec<Label> = Vec::with_capacity(af.nb_argument);

    Vec::new()
}

fn propagateDefense(af : &ArgumentationFramework, labelling : &Vec<Label>) -> Vec<Label> {
    Vec::new()
}
fn sameLabelling(labelling : &Vec<Label>, newOne : &Vec<Label>) -> bool {
    false
}