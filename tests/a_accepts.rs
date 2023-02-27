use l1_dfa::{DFA};

#[test]
fn empty_string() {
   let d = DFA {
      states: vec![true],
      transitions: std::collections::HashMap::new(),
   };

   assert!( d.accepts(&"".to_string()) );
   assert!( !d.accepts(&"a".to_string()) );
}
