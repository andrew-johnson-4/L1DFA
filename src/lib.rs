//Manually Transpiled, TODO generate from L1 specification

pub struct DFA {
  pub states: Vec<bool>,
  pub transitions: std::collections::HashMap<(u64,char),u64>,
}

impl DFA {
  pub fn accepts(self: &DFA, s: &str) -> bool {
    let d = self;
    let mut at:u64 = 0;
    for c in s.chars() {
      if d.transitions.contains_key(&(at,c)) {
         at = d.transitions.get(&(at,c)).unwrap().clone();
      } else {
         return false;
      }
    };
    d.states[at as usize]
  }
}

impl DFA {
   pub fn intersect(self: &DFA, right: &DFA) -> DFA {
      let left = self;
      let mut p = DFA {
         states: vec![false; left.states.len() * right.states.len()],
         transitions: std::collections::HashMap::new(),
      };
      for pi in 0..p.states.len() {
         let li = pi / right.states.len();
         let ri = pi % right.states.len();
         if left.states[li] && right.states[ri] {
            p.states[pi] = true;
         }
      }
      for ((ls,lc),lt) in left.transitions.iter() {
      for ((rs,rc),rt) in right.transitions.iter() {
      if lc == rc {
         let ps = ls * (right.states.len() as u64) + rs;
         let pt = lt * (right.states.len() as u64) + rt;
         p.transitions.insert((ps,*lc),pt);
      }}}
      p
   }
}

impl DFA {
   pub fn complement(self: &DFA) -> DFA {
      let d = self;
      DFA {
         states: d.states.iter().map(|c| !c).collect::<Vec<bool>>(),
         transitions: self.transitions.clone(),
      }
   }
}
