pub struct ArgumentationFramework {
    pub af_attacker : Vec<Vec<i32>>,
	pub af_attackee : Vec<Vec<i32>>,
    pub nb_argument : usize
}

impl ArgumentationFramework {
    pub fn new(nb_arg : usize) -> Self {
        let af_attackee = vec![Vec::new();nb_arg];
        let af_attacker = vec![Vec::new();nb_arg];
        Self { af_attackee , af_attacker, nb_argument : nb_arg }
    }
    pub fn add_attack(&mut self, attacker : i32, target : i32) {
        self.af_attacker[(target-1) as usize].push(attacker-1);
        self.af_attackee[(attacker-1) as usize].push(target-1);
    }
}