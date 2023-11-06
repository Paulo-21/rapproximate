use crate::{parser::Task, graph::ArgumentationFramework};
use crate::extensionsemantics::SimpleGroundedSemanticsSolver::Label;
use std::collections::HashSet;

pub fn solve(af : &ArgumentationFramework, task : &Task) -> Vec<usize> {

    let problem_type = task.problem;
    let (mut label_in_final, mut label_undec) = initLabelling(af);
    let mut label_in_new = label_in_final.clone();
    let mut label_out = vec![false;af.nb_argument];
    let mut label_undex_index : Vec<usize> = Vec::with_capacity(af.nb_argument);
    let mut hasChanged = true;
    while hasChanged {
        hasChanged = false;
        let mut n_label_in_new = Vec::with_capacity(30);
        for i in &label_in_new {
            for argument in &af.af_attackee[*i] {
                label_out[*argument as usize] = true;
                label_undec.remove(&(*argument as usize));
                //remove_element(&mut label_undec, *argument as usize);
                hasChanged = true;
            }
        }
        let mut to_remove = Vec::with_capacity(30);
        for el in &label_undec {
            if allAttackersAreOut(af, &label_out, *el) {
                n_label_in_new.push(*el);
                label_in_final.push(*el);
                to_remove.push(*el);
                hasChanged = true;
            }
        }
        for remove in to_remove {
            label_undec.remove(&remove);
        }
        label_in_new = n_label_in_new;
    }
    label_in_final
}
fn remove_element(tab : &mut Vec<usize>, element : usize) {
    let index = tab.iter().position(|x| *x == element).unwrap();
    tab.remove(index);
}

fn initLabelling(af : &ArgumentationFramework) -> (Vec<usize>, HashSet<usize>,) {
    let mut label_in : Vec<usize> = Vec::new();//(af.nb_argument/2);
    //let mut label_undec : Vec<usize> = Vec::with_capacity(af.nb_argument);
    let mut label_undec = HashSet::with_capacity(af.nb_argument);
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			label_in.push(i);
		} else {
		    label_undec.insert(i);
		}
	}
    (label_in, label_undec)
}

fn allAttackersAreOut(af : &ArgumentationFramework, labelling : &Vec<bool>, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if labelling[(*attacker) as usize] {
			return false;
		}
	}
	true
}