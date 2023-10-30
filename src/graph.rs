use std::collections::HashMap;

pub struct ArgumentationFramework {
    af_attacker : Vec<i16>,
	af_attackee : Vec<i16>,
	argument_names : HashMap<String, i16>,
}

impl ArgumentationFramework {
    pub fn new(nb_arg : usize) -> Self {
        let af_attackee = vec![0;nb_arg];
        let af_attacker = vec![0;nb_arg];
        let argument_names = HashMap::new();
        Self { af_attackee , af_attacker, argument_names }
    }
}