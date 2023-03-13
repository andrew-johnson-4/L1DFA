use l1_dfa::{DFA};

#[test]
fn intersect_1() {
   let mut left = DFA {
      states: vec![false,false,true,false],
      transitions: std::collections::HashMap::new(),
   };
   left.transitions.insert((0,'a'),1);
   left.transitions.insert((1,'a'),2);
   left.transitions.insert((2,'a'),3);
   left.transitions.insert((3,'a'),3);
   left.transitions.insert((0,'b'),0);
   left.transitions.insert((1,'b'),1);
   left.transitions.insert((2,'b'),2);
   left.transitions.insert((3,'b'),3);

   assert!( !left.accepts("") );
   assert!( !left.accepts("a") );
   assert!( !left.accepts("b") );
   assert!(  left.accepts("aa") );
   assert!( !left.accepts("ab") );
   assert!( !left.accepts("aaa") );
   assert!(  left.accepts("aab") );
   assert!( !left.accepts("aaaa") );
   assert!( !left.accepts("aaab") );
   assert!(  left.accepts("aabb") );
   assert!(  left.accepts("aabbb") );
   assert!(  left.accepts("babab") );
   assert!(  left.accepts("ababb") );
   assert!(  left.accepts("ababbb") );
   assert!(  left.accepts("babab") );
   assert!(  left.accepts("bababbb") );
   assert!(  left.accepts("bbabbabbb") );

   let mut right = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   right.transitions.insert((0,'a'),0);
   right.transitions.insert((1,'a'),1);
   right.transitions.insert((2,'a'),2);
   right.transitions.insert((0,'b'),1);
   right.transitions.insert((1,'b'),2);
   right.transitions.insert((2,'b'),2);

   assert!( !right.accepts("") );
   assert!( !right.accepts("a") );
   assert!( !right.accepts("b") );
   assert!( !right.accepts("aa") );
   assert!( !right.accepts("ab") );
   assert!( !right.accepts("ba") );
   assert!(  right.accepts("bb") );
   assert!(  right.accepts("abb") );
   assert!(  right.accepts("bab") );
   assert!(  right.accepts("bba") );
   assert!(  right.accepts("bbb") );
   assert!(  right.accepts("bbbba") );
   assert!(  right.accepts("abbab") );
 
   let product = left.intersect(&right);
   assert!( product.states[0] == false  ); //(0,0)
   assert!( product.states[1] == false  ); //(0,1)
   assert!( product.states[2] == false  ); //(0,2)
   assert!( product.states[3] == false  ); //(1,0)
   assert!( product.states[4] == false  ); //(1,1)
   assert!( product.states[5] == false  ); //(1,2)
   assert!( product.states[6] == false  ); //(2,0)
   assert!( product.states[7] == false  ); //(2,1)
   assert!( product.states[8] == true   ); //(2,2)
   assert!( product.states[9] == false  ); //(3,0)
   assert!( product.states[10] == false ); //(3,1)
   assert!( product.states[11] == false ); //(3,2)

   assert!( !product.accepts("") );
   assert!( product.accepts("aabb") );
   assert!( product.accepts("abba") );
   assert!( product.accepts("bbaa") );
   assert!( product.accepts("bbbaa") );
   assert!( !product.accepts("abbaa") );

   /*
   assert_eq!( product.transitions.len(), 24 );
   assert_eq!( product.transitions.get(&(0,'a')), Some(&3) );
   assert_eq!( product.transitions.get(&(3,'a')), Some(&6) );
   assert_eq!( product.transitions.get(&(6,'a')), Some(&9) );
   assert_eq!( product.transitions.get(&(9,'a')), Some(&9) );
   assert_eq!( product.transitions.get(&(1,'a')), Some(&4) );
   assert_eq!( product.transitions.get(&(4,'a')), Some(&7) );
   assert_eq!( product.transitions.get(&(7,'a')), Some(&10) );
   assert_eq!( product.transitions.get(&(10,'a')), Some(&10) );
   assert_eq!( product.transitions.get(&(2,'a')), Some(&5) );
   assert_eq!( product.transitions.get(&(5,'a')), Some(&8) );
   assert_eq!( product.transitions.get(&(8,'a')), Some(&11) );
   assert_eq!( product.transitions.get(&(11,'a')), Some(&11) );
   assert_eq!( product.transitions.get(&(0,'b')), Some(&1) );
   assert_eq!( product.transitions.get(&(1,'b')), Some(&2) );
   assert_eq!( product.transitions.get(&(2,'b')), Some(&2) );
   assert_eq!( product.transitions.get(&(3,'b')), Some(&4) );
   assert_eq!( product.transitions.get(&(4,'b')), Some(&5) );
   assert_eq!( product.transitions.get(&(5,'b')), Some(&5) );
   assert_eq!( product.transitions.get(&(6,'b')), Some(&7) );
   assert_eq!( product.transitions.get(&(7,'b')), Some(&8) );
   assert_eq!( product.transitions.get(&(8,'b')), Some(&8) );
   assert_eq!( product.transitions.get(&(9,'b')), Some(&10) );
   assert_eq!( product.transitions.get(&(10,'b')), Some(&11) );
   assert_eq!( product.transitions.get(&(11,'b')), Some(&11) );
   */
}

#[test]
fn intersect_2() {
   let mut left = DFA {
      states: vec![true,false],
      transitions: std::collections::HashMap::new(),
   };
   left.transitions.insert((0,'b'),0);
   left.transitions.insert((1,'b'),1);
   left.transitions.insert((0,'a'),1);
   left.transitions.insert((1,'a'),0);

   let mut right = DFA {
      states: vec![true,false,false],
      transitions: std::collections::HashMap::new(),
   };
   right.transitions.insert((0,'a'),1);
   right.transitions.insert((1,'a'),2);
   right.transitions.insert((2,'a'),2);
   right.transitions.insert((0,'b'),0);
   right.transitions.insert((1,'b'),0);
   right.transitions.insert((2,'b'),2);
 
   let product = left.intersect(&right);
   assert!( product.states[0] == true   ); //(0,0)
   assert!( product.states[1] == false  ); //(0,1)
   assert!( product.states[2] == false  ); //(0,2)
   assert!( product.states[3] == false  ); //(1,0)
   assert!( product.states[4] == false  ); //(1,1)
   assert!( product.states[5] == false  ); //(1,2)

   assert_eq!( product.transitions.len(), 12 );
   assert_eq!( product.transitions.get(&(0,'a')), Some(&4) ); //(0,0) -> (1,1)
   assert_eq!( product.transitions.get(&(3,'a')), Some(&1) ); //(1,0) -> (0,1)
   assert_eq!( product.transitions.get(&(1,'a')), Some(&5) ); //(0,1) -> (1,2)
   assert_eq!( product.transitions.get(&(4,'a')), Some(&2) ); //(1,1) -> (0,2)
   assert_eq!( product.transitions.get(&(2,'a')), Some(&5) ); //(0,2) -> (1,2)
   assert_eq!( product.transitions.get(&(5,'a')), Some(&2) ); //(1,2) -> (0,2)

   assert_eq!( product.transitions.get(&(0,'b')), Some(&0) ); //(0,0) -> (0,0)
   assert_eq!( product.transitions.get(&(3,'b')), Some(&3) ); //(1,0) -> (1,0)
   assert_eq!( product.transitions.get(&(1,'b')), Some(&0) ); //(0,1) -> (0,0)
   assert_eq!( product.transitions.get(&(4,'b')), Some(&3) ); //(1,1) -> (1,0)
   assert_eq!( product.transitions.get(&(2,'b')), Some(&2) ); //(0,2) -> (0,2)
   assert_eq!( product.transitions.get(&(5,'b')), Some(&5) ); //(1,2) -> (1,2)
}
