use l1_dfa::{try_parse};

#[test]
fn re_abc() {
   let r = try_parse("abc").unwrap();

   assert!( !r.accepts("") );
   assert!( !r.accepts("a") );
   assert!( !r.accepts("ab") );
   assert!( r.accepts("abc") );
   assert!( !r.accepts("abcd") );
}

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
