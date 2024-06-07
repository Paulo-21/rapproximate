pub struct ArgumentationFramework {
    pub af_attacker : Vec<Vec<u32>>,
	pub af_attackee : Vec<Vec<u32>>,
    pub nb_argument : usize
}

impl ArgumentationFramework {
    pub fn new(nb_arg : usize) -> Self {
        let af_attackee = vec![Vec::new();nb_arg];
        let af_attacker = vec![Vec::new();nb_arg];
        Self { af_attackee , af_attacker, nb_argument : nb_arg }
    }
    pub fn add_attack(&mut self, mut attacker : u32, mut target : u32) {
        attacker -= 1;
        target -= 1;
        self.af_attacker[target as usize].push(attacker);
        self.af_attackee[attacker as usize].push(target);
    }
    pub fn inDegree(&self, argument : usize) -> usize {
        self.af_attacker[argument].len()
    }
    pub fn outDegree(&self, argument : usize) -> usize {
        self.af_attackee[argument].len()
    }
}