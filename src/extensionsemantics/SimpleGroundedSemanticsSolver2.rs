use crate::{parser::Task, graph::ArgumentationFramework};
use crate::extensionsemantics::SimpleGroundedSemanticsSolver::Label;

use std::collections::BTreeSet;
use std::io::{stdout, Write};
use std::time::Instant;

pub fn solve(af : &ArgumentationFramework, task : &Task) -> Vec<usize> {

    let problem_type = task.problem;
    let mut label_in_final = initLabelling2(af);
    let mut label_in_new = label_in_final.clone();
    let mut label_out = vec![false;af.nb_argument];
    let mut suspect_in = Vec::with_capacity(af.nb_argument);
    let mut hasChanged = true;
    while hasChanged {
        hasChanged = false;
        let mut n_label_in_new = Vec::with_capacity(af.nb_argument/10);
        for i in &label_in_new {
            for argument in &af.af_attackee[*i] {
                if !label_out[*argument as usize] {
                    label_out[*argument as usize] = true;
                    suspect_in.push(&af.af_attackee[*argument as usize]);
                    hasChanged = true;
                }
            }   
        }
        for el in &suspect_in {
            for index_of_suspect in &**el {
                if allAttackersAreOut(af, &label_out, *index_of_suspect as usize) {
                    n_label_in_new.push(*index_of_suspect as usize);
                    label_in_final.push(*index_of_suspect as usize);
                    hasChanged = true;
                }
            }
        }
        suspect_in.clear();
        label_in_new = n_label_in_new;
    }
    label_in_final
}


fn initLabelling(af : &ArgumentationFramework) -> Vec<Label> {
    let mut labelling : Vec<Label> = Vec::with_capacity(af.nb_argument);
    //let mut label_undec = BTreeSet::new();
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
            labelling.push(Label::IN);
		}
        else {
            labelling.push(Label::UNDEC);
        }
	}
    labelling
}
fn initLabelling2(af : &ArgumentationFramework) -> (Vec<usize>) {
    let mut label_in : Vec<usize> = Vec::with_capacity(af.nb_argument);
    
    //let mut label_undec = BTreeSet::new();
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			label_in.push(i);
		}
	}
    (label_in)
}

fn allAttackersAreOut(af : &ArgumentationFramework, labelling : &Vec<bool>, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if !labelling[(*attacker) as usize] {
			return false;
		}
	}
	true
}