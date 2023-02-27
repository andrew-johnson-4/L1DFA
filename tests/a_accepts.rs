use l1_dfa::{DFA};

#[test]
fn empty_string() {
   let d = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };

   assert!( d.accepts("") );
   assert!( !d.accepts("a") );
}
