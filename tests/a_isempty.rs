use l1_dfa::{DFA};

#[test]
fn not_empty_string() {
   let d = DFA {
      states: vec![false],
      transitions: std::collections::HashMap::new(),
   };

   assert!( d.is_empty() );
}

#[test]
fn empty_string() {
   let d = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };

   assert!( !d.is_empty() );
}

#[test]
fn aaa_string() {
   let mut d = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   d.transitions.insert((0,'a'),1);

   assert!( !d.is_empty() );
}

#[test]
fn aaa_string2() {
   let mut d = DFA {
      states: vec![true,false],
      transitions: std::collections::HashMap::new(),
   };
   d.transitions.insert((0,'a'),1);

   assert!( !d.is_empty() );
}

#[test]
fn aaa_string3() {
   let mut d = DFA {
      states: vec![false,false],
      transitions: std::collections::HashMap::new(),
   };
   d.transitions.insert((0,'a'),1);

   assert!( d.is_empty() );
}
