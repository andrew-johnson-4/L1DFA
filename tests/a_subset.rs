use l1_dfa::{DFA};

#[test]
fn subset_ea1() {
   let l = DFA {
      states: vec![false],
      transitions: std::collections::HashMap::new(),
   };
   let r = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };

   assert!( l.is_subset_of(&r) );
   assert!( !r.is_subset_of(&l) );
}

#[test]
fn subset_ea2() {
   let l = DFA {
      states: vec![false],
      transitions: std::collections::HashMap::new(),
   };
   let mut r = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };
   r.transitions.insert((0,'a'),0);

   assert!( l.is_subset_of(&r) );
   assert!( !r.is_subset_of(&l) );
}

#[test]
fn subset_ea3() {
   let mut l = DFA {
      states: vec![true,false],
      transitions: std::collections::HashMap::new(),
   };
   l.transitions.insert((0,'a'),1);
   let mut r = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };
   r.transitions.insert((0,'a'),0);

   assert!( l.is_subset_of(&r) );
   assert!( !r.is_subset_of(&l) );
}

#[test]
fn subset_ea4() {
   let mut l = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   l.transitions.insert((0,'a'),1);
   let mut r = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };
   r.transitions.insert((0,'a'),0);

   assert!( l.is_subset_of(&r) );
   assert!( !r.is_subset_of(&l) );
}

#[test]
fn subset_ea5() {
   let mut l = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   l.transitions.insert((0,'a'),1);
   l.transitions.insert((1,'a'),2);
   l.transitions.insert((2,'a'),2);
   let mut r = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   r.transitions.insert((0,'a'),1);
   r.transitions.insert((1,'a'),1);

   assert!( l.is_subset_of(&r) );
   assert!( !r.is_subset_of(&l) );
}

#[test]
fn subset_ea6() {
   let mut l = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   l.transitions.insert((0,'a'),1);
   l.transitions.insert((1,'a'),1);
   let mut r = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   r.transitions.insert((0,'a'),1);
   r.transitions.insert((1,'a'),1);

   assert!( l.is_subset_of(&r) );
   assert!( r.is_subset_of(&l) );
}









