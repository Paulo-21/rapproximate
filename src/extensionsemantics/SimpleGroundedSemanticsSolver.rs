use crate::{parser::Format, graph::ArgumentationFramework};

enum Label {
    IN, OUT, UNDEC
}

pub fn solve(problem_type : Format, af : ArgumentationFramework) {

    let mut labelling = initLabelling(&af);

    let mut hasChanged = true;
    while hasChanged {
        let new_labelling = propagateDefense(&af, &labelling);
        hasChanged = !sameLabelling(&new_labelling, &labelling);
        labelling = new_labelling;
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