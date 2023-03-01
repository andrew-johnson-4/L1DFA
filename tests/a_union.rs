use l1_dfa::{try_parse};

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

