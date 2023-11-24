use crate::{cli::Task, graph::ArgumentationFramework};
use crate::extensionsemantics::SimpleGroundedSemanticsSolver::Label;
use fixedbitset::{self, FixedBitSet};

pub fn solve(af : &ArgumentationFramework, task : &Task) -> Vec<usize> {

    let mut label_in_final = initLabelling2(af);
    let mut label_in_new = label_in_final.clone();
    let mut label_out = vec![false;af.nb_argument];
    let mut suspect_in = Vec::with_capacity(af.nb_argument);
    let mut hasChanged = true;
    let mut n_label_in_new = Vec::with_capacity(af.nb_argument/10);
    while hasChanged {
        hasChanged = false;

        for i in &label_in_new {
            for argument in &af.af_attackee[*i] {
                if !label_out[*argument as usize] {
                    label_out[*argument as usize] = true;
                    suspect_in.push(&af.af_attackee[*argument as usize]);
                }
            }
        }
        for el in &suspect_in {
            for index_of_suspect in &**el {
                if !label_out[*index_of_suspect as usize] && allAttackersAreOut(af, &label_out, *index_of_suspect as usize)  {
                    n_label_in_new.push(*index_of_suspect as usize);
                    label_in_final.push(*index_of_suspect as usize);
                    hasChanged = true;
                    /*if task.argument == *index_of_suspect as usize {
                        return label_in_final;
                    }*/
                }
            }
        }
        suspect_in.clear();
        label_in_new.clear();
        std::mem::swap(&mut label_in_new, &mut n_label_in_new);
    }
    label_in_final
}
pub fn solve_with_bitset(af : &ArgumentationFramework, _task : &Task) -> Vec<usize> {

    let mut label_in_final = initLabelling2(af);
    let mut label_in_new = label_in_final.clone();
    let mut label_out = FixedBitSet::with_capacity(af.nb_argument);
    let mut suspect_in = Vec::with_capacity(af.nb_argument);
    let mut hasChanged = true;
    let mut n_label_in_new = Vec::with_capacity(af.nb_argument);
    while hasChanged {
        hasChanged = false;
        for i in &label_in_new {
            for argument in &af.af_attackee[*i] {
                if !label_out[*argument as usize] {
                    label_out.insert(*argument as usize);
                    suspect_in.push(&af.af_attackee[*argument as usize]);
                }
            }
        }
        for el in &suspect_in {
            for index_of_suspect in &**el {
                if allAttackersAreOut2(af, &label_out, *index_of_suspect as usize) {
                    n_label_in_new.push(*index_of_suspect as usize);
                    label_in_final.push(*index_of_suspect as usize);
                    hasChanged = true;
                }
            }
        }
        suspect_in.clear();
        label_in_new.clear();
        std::mem::swap(&mut label_in_new, &mut n_label_in_new);
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
fn initLabelling2(af : &ArgumentationFramework) -> Vec<usize> {
    let mut label_in : Vec<usize> = Vec::with_capacity(af.nb_argument);
    
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			label_in.push(i);
		}
	}
    label_in
}
fn initLabelling_bitset(af : &ArgumentationFramework) -> FixedBitSet {
    let mut label_in = FixedBitSet::with_capacity(af.nb_argument);
    
    //let mut label_undec = BTreeSet::new();
    for i in 0..af.nb_argument {
	    if af.af_attacker[i].is_empty() {
			label_in.insert(i);
		}
	}
    label_in
}

fn allAttackersAreOut(af : &ArgumentationFramework, labelling : &Vec<bool>, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if !labelling[(*attacker) as usize] {
			return false;
		}
	}
	true
}
fn allAttackersAreOut2(af : &ArgumentationFramework, labelling : &FixedBitSet, index : usize) -> bool {
    for attacker in &af.af_attacker[index] {
		if !labelling[(*attacker) as usize] {
			return false;
		}
	}
	true
}