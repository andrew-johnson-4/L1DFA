use l1_dfa::{DFA,try_parse};

#[test]
fn re_ab_or_cd() {
   let r = try_parse("ab|cd").unwrap();

   assert!( !r.accepts("") );
   assert!( !r.accepts("a") );
   assert!( r.accepts("ab") );
   assert!( !r.accepts("abc") );
   assert!( !r.accepts("abcd") );
   assert!( !r.accepts("") );
   assert!( !r.accepts("c") );
   assert!( r.accepts("cd") );
}

#[test]
fn re_abc_or_bcd() {
   let r = try_parse("abc|bcd").unwrap();

   assert!( !r.accepts("") );
   assert!( !r.accepts("a") );
   assert!( !r.accepts("ab") );
   assert!( r.accepts("abc") );
   assert!( !r.accepts("abcd") );
   assert!( !r.accepts("") );
   assert!( !r.accepts("b") );
   assert!( !r.accepts("bc") );
   assert!( r.accepts("bcd") );
   assert!( !r.accepts("abb") );
}

#[test]
fn ababcd() {
   let mut abab = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   abab.transitions.insert((0,'a'),1);
   abab.transitions.insert((1,'b'),2);
   abab.transitions.insert((2,'a'),1);
   assert!( abab.accepts("ab") );
   assert!( abab.accepts("abab") );

   let mut abcd = DFA {
      states: vec![false,false,false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   abcd.transitions.insert((0,'a'),1);
   abcd.transitions.insert((1,'b'),2);
   abcd.transitions.insert((2,'c'),3);
   abcd.transitions.insert((3,'d'),4);
   assert!( abcd.accepts("abcd") );

   let ababcd = abab.union(&abcd);
   assert!( !ababcd.accepts("") );
   assert!( !ababcd.accepts("a") );
   assert!(  ababcd.accepts("ab") );
   assert!( !ababcd.accepts("aba") );
   assert!(  ababcd.accepts("abab") );
   assert!( !ababcd.accepts("abc") );
   assert!(  ababcd.accepts("abcd") );
   assert!( !ababcd.accepts("ababcd") );
}

#[test]
fn ab_tough() {
   let mut abab = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   abab.transitions.insert((0,'a'),1);
   abab.transitions.insert((1,'b'),2);
   abab.transitions.insert((2,'a'),1);
   assert!( abab.accepts("ab") );
   assert!( abab.accepts("abab") );

   let mut aa = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   aa.transitions.insert((0,'a'),1);
   aa.transitions.insert((1,'a'),2);
   assert!( aa.accepts("aa") );

   let ababaa = abab.union(&aa);
   assert!( !ababaa.accepts("") );
   assert!( !ababaa.accepts("a") );
   assert!(  ababaa.accepts("aa") );
   assert!(  ababaa.accepts("ab") );
   assert!( !ababaa.accepts("aba") );
   assert!(  ababaa.accepts("abab") );
   assert!( !ababaa.accepts("abaa") );
}

#[test]
fn ab_tough2() {
   let mut ab = DFA {
      states: vec![true,false],
      transitions: std::collections::HashMap::new(),
   };
   ab.transitions.insert((0,'a'),1);
   ab.transitions.insert((1,'b'),0);
   assert!( ab.accepts("") );
   assert!( ab.accepts("ab") );
   assert!( ab.accepts("abab") );

   let mut aa = DFA {
      states: vec![false,false,true],
      transitions: std::collections::HashMap::new(),
   };
   aa.transitions.insert((0,'a'),1);
   aa.transitions.insert((1,'a'),2);
   assert!( aa.accepts("aa") );

   let ababaa = ab.union(&aa);
   assert!(  ababaa.accepts("") );
   assert!( !ababaa.accepts("a") );
   assert!(  ababaa.accepts("aa") );
   assert!(  ababaa.accepts("ab") );
   assert!( !ababaa.accepts("aba") );
   assert!(  ababaa.accepts("abab") );
}

#[test]
fn re_aabbccdd() {
   let a = try_parse("(ab)*cd").unwrap();
   assert!( !a.accepts("") );
   assert!( !a.accepts("c") );
   assert!(  a.accepts("cd") );
   assert!( !a.accepts("a") );
   assert!( !a.accepts("ab") );
   assert!( !a.accepts("abc") );
   assert!(  a.accepts("abcd") );
   assert!( !a.accepts("aabcd") );
   assert!(  a.accepts("ababcd") );

   let b = try_parse("a(bc)*d").unwrap();
   assert!( !a.accepts("") );
   assert!( !a.accepts("a") );
   assert!(  a.accepts("ad") );
   assert!( !a.accepts("ab") );
   assert!( !a.accepts("abc") );
   assert!(  a.accepts("abcd") );
   assert!( !a.accepts("abbcd") );
   assert!(  a.accepts("abcbcd") );

   let c = try_parse("ab(cd)*").unwrap();
   assert!( !a.accepts("") );
   assert!( !a.accepts("a") );
   assert!(  a.accepts("ab") );
   assert!( !a.accepts("abc") );
   assert!(  a.accepts("abcd") );
   assert!( !a.accepts("abcdc") );
   assert!(  a.accepts("abcdcd") );

   let r = a.union(&b).union(&c);

   assert!( !r.accepts("") );
   assert!( !r.accepts("a") );
   assert!( !r.accepts("b") );
   assert!( !r.accepts("c") );
   assert!( !r.accepts("d") );

   assert!(  r.accepts("ab") );
   assert!( !r.accepts("ac") );
   assert!(  r.accepts("ad") );
   assert!( !r.accepts("bc") );
   assert!( !r.accepts("bd") );
   assert!(  r.accepts("cd") );

   for c1 in "abcd".chars() {
   for c2 in "abcd".chars() {
   for c3 in "abcd".chars() {
      let cs: String = vec![c1,c2,c3].into_iter().collect();
      assert!( !r.accepts(&cs) );
   for c4 in "abcd".chars() {
      let cs: String = vec![c1,c2,c3,c4].into_iter().collect();
      if cs == "abcd" {
         assert!( r.accepts(&cs) );
      } else {
         assert!( !r.accepts(&cs) );
      }
   }}}}
}

