use l1_dfa::{DFA};

#[test]
fn complement_aaa() {
   let mut d = DFA {
      states: vec![false,true],
      transitions: std::collections::HashMap::new(),
   };
   d.transitions.insert((0,'a'),1);
   d.transitions.insert((1,'a'),1);

   assert!( !d.accepts("") );
   assert!( d.accepts("a") );
   assert!( d.accepts("aa") );
   assert!( d.accepts("aaa") );

   let d = d.complement();
   assert!( d.accepts("") );
   assert!( !d.accepts("a") );
   assert!( !d.accepts("aa") );
   assert!( !d.accepts("aaa") );
}
