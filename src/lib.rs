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
