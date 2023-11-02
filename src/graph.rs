use std::collections::HashMap;

pub struct ArgumentationFramework {
    af_attacker : Vec<Vec<i32>>,
	af_attackee : Vec<Vec<i32>>,
}

impl ArgumentationFramework {
    pub fn new(nb_arg : usize) -> Self {
        let af_attackee = vec![Vec::new();nb_arg];
        let af_attacker = vec![Vec::new();nb_arg];
        //let argument_names = HashMap::new();
        Self { af_attackee , af_attacker}//, argument_names }
    }
    pub fn add_attack(&mut self, attacker : i32, target : i32) {
        self.af_attacker[target as usize].push(attacker);
        self.af_attackee[attacker as usize].push(target);
    }
}