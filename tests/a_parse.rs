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
