use crate::{parser::Task, graph::ArgumentationFramework};
use crate::extensionsemantics::SimpleGroundedSemanticsSolver::Label;

use std::collections::BTreeSet;
use std::io::{stdout, Write};
use std::time::Instant;

pub fn solve(af : &ArgumentationFramework, task : &Task) -> Vec<usize> {

    let problem_type = task.problem;
    //let start = Instant::now();
    let (mut label_in_final, mut label_undec) = initLabelling(af);
    //println!("{}", start.elapsed().as_millis() as f32/1000.0);
    let mut label_in_new = label_in_final.clone();
    //stdout().flush();
    let mut label_out = vec![false;af.nb_argument];
    let mut hasChanged = true;
    while hasChanged {
        hasChanged = false;
        let mut n_label_in_new = Vec::with_capacity(af.nb_argument/10);
        for i in &label_in_new {
            for argument in &af.af_attackee[*i] {
                label_out[*argument as usize] = true;
                label_undec[*argument as usize] = false;
                //label_undec.remove(&(*argument as usize));
                //remove_element(&mut label_undec, *argument as usize);
                hasChanged = true;
            }
        }
        //let mut to_remove = Vec::with_capacity(af.nb_argument/2);
        let mut i = 0;
        for el in &mut label_undec {
            if *el && allAttackersAreOut(af, &label_out, i) {
                n_label_in_new.push(i);
                label_in_final.push(i);
                *el = false;
                //to_remove.push(*el);
                hasChanged = true;
            }
            i+=1;
        }
        /*for remove in to_remove {
            label_undec.remove(&remove);
        }*/
        //label_in_new = n_label_in_new;
    }
    label_in_final
}


fn initLabelling(af : &ArgumentationFramework) -> (Vec<usize>, Vec<bool>,) {
    let mut label_in : Vec<usize> = Vec::with_capacity(af.nb_argument/2);
    let mut label_undec = vec![false;af.nb_argument];
    //let mut label_undec = BTreeSet::new();
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			label_in.push(i);
		    label_undec[i] = false;
		}
	}
    (label_in, label_undec)
}

fn allAttackersAreOut(af : &ArgumentationFramework, labelling : &Vec<bool>, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if !labelling[(*attacker) as usize] {
			return false;
		}
	}
	true
}