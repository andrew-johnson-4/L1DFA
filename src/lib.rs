//Manually Transpiled, TODO generate from L1 specification

pub struct DFA {
  pub states: Vec<bool>,
  pub transitions: std::collections::HashMap<(u64,char),u64>,
}

impl DFA {
  pub fn print(self: &DFA) {
    println!("STATES:");
    for si in 0..self.states.len() {
      println!("  {}: {}", si, self.states[si]); 
    }
    for ((s,c),t) in self.transitions.iter() {
      println!("  {}:{} -> {}", s, c, t);
    }
  }

  pub fn accepts(self: &DFA, s: &str) -> bool {
    let d = self;
    let mut at:u64 = 0;
    for c in s.chars() {
      if d.transitions.contains_key(&(at,c)) {
         at = d.transitions.get(&(at,c)).unwrap().clone();
      } else {
         return false;
      }
    };
    d.states[at as usize]
  }

  pub fn intersect(self: &DFA, right: &DFA) -> DFA {
    let left = self;
    let mut p = DFA {
      states: vec![false; left.states.len() * right.states.len()],
      transitions: std::collections::HashMap::new(),
    };
    for pi in 0..p.states.len() {
      let li = pi / right.states.len();
      let ri = pi % right.states.len();
      if left.states[li] && right.states[ri] {
        p.states[pi] = true;
      }
    }
    for ((ls,lc),lt) in left.transitions.iter() {
    for ((rs,rc),rt) in right.transitions.iter() {
    if lc==rc {
      let ps = ls * (right.states.len() as u64) + rs;
      let pt = lt * (right.states.len() as u64) + rt;
      p.transitions.insert((ps,*lc),pt);
    }}}
    p
  }

  pub fn complement(self: &DFA) -> DFA {
    let d = self;
    let mut states = d.states.iter().map(|c| !c).collect::<Vec<bool>>();
    let mut alphabet = std::collections::HashSet::new();
    let mut transitions = std::collections::HashMap::new();
    for ((_,c),_) in self.transitions.iter() {
       alphabet.insert(*c);
    }
    let mut null_transition = false;
    for s in 0..(self.transitions.len() as u64) {
    for c in alphabet.iter() {
       if let Some(t) = self.transitions.get(&(s,*c)) {
          transitions.insert((s,*c),*t);
       } else {
          null_transition = true;
          transitions.insert((s,*c),self.transitions.len() as u64);
       }
    }}
    if null_transition {
       let ni = self.transitions.len() as u64;
       states.push(true);
       for c in alphabet.iter() {
          transitions.insert((ni,*c),ni);
       }
    }
    DFA {
      states: states,
      transitions: transitions,
    }
  }

  pub fn union(self: &DFA, right: &DFA) -> DFA {
    //a union state is either rejected, on the left, on the right, or on both
    //this algorithm still yields a O(xy) upper bound
    //TODO: implement this algorithm as a fallback until the simpler algorithm is better understood
    let left = self;
    let mut p = DFA {
      states: vec![false; left.states.len() * right.states.len()],
      transitions: std::collections::HashMap::new(),
    };
    for pi in 0..p.states.len() {
      let li = pi / right.states.len();
      let ri = pi % right.states.len();
      if left.states[li] || right.states[ri] {
        p.states[pi] = true;
      }
    }
    for ((ls,lc),lt) in left.transitions.iter() {
    for ((rs,rc),rt) in right.transitions.iter() {
      let ps = ls * (right.states.len() as u64) + rs;
      let pt = lt * (right.states.len() as u64) + rt;
      p.transitions.insert((ps,*lc),pt);
      p.transitions.insert((ps,*rc),pt);
    }}
    p
  }

  pub fn is_empty(self: &DFA) -> bool {
    let mut reached = 0;
    let mut reach = std::collections::HashSet::new();
    reach.insert(0);
    while reach.len() > reached {
      reached = reach.len();
      for ((ls,_lc),lt) in self.transitions.iter() {
      if reach.contains(ls) {
        reach.insert(*lt);
      }}
    }
    for ri in reach.iter() {
    if self.states[*ri as usize] {
      return false;
    }}
    true
  }

  pub fn is_subset_of(self: &DFA, right: &DFA) -> bool {
     self.intersect( &right.complement() ).is_empty()
  }

  pub fn concatenate(self: &DFA, _right: &DFA) -> DFA {
     unimplemented!("DFA::concatenate")
  }
}

use regex_syntax::Parser;
use regex_syntax::hir::{self,HirKind};

pub fn try_parse(regex: &str) -> Option<DFA> {
   let Ok(hir) = Parser::new().parse(regex)
   else { return None; };
   Some(compile_ast(hir.kind()))
}

fn compile_literal(cs: &str) -> DFA {
   let cs = cs.chars().collect::<Vec<char>>();
   let mut states = vec![false; cs.len()+1];
   states[cs.len()] = true;
   let mut transitions = std::collections::HashMap::new();
   for ci in 0..cs.len() {
      transitions.insert((ci as u64,cs[ci]),(ci+1) as u64);
   }
   DFA {
      states: states,
      transitions: transitions,
   }
}

fn compile_ast(hir: &HirKind) -> DFA {
   match hir {
      HirKind::Empty => unimplemented!("try_compile Empty Regex"),
      HirKind::Literal(_l) => unimplemented!("try_compile Literal Regex"),
      HirKind::Class(_c) => unimplemented!("try_compile Class Regex"),
      HirKind::Anchor(_a) => unimplemented!("try_compile Anchor Regex"),
      HirKind::WordBoundary(_wb) => unimplemented!("try_compile Word Boundary Regex"),
      HirKind::Repetition(_r) => unimplemented!("try_compile Repetition Regex"),
      HirKind::Group(_g) => unimplemented!("try_compile Group Regex"),
      HirKind::Concat(cs) => {
         enum C { S(String), K(HirKind) }
         let mut ds: Vec<C> = Vec::new();
         let mut sbuf = "".to_string();
         for c in cs.iter() {
            let ck = c.kind();
            if let HirKind::Literal(kl) = ck {
            match kl {
               hir::Literal::Unicode(c) => { sbuf.push(*c); }
               hir::Literal::Byte(b) => { sbuf.push(*b as char); }
            }} else {
               if sbuf.len()>0 {
                  ds.push(C::S(sbuf.clone()));
                  sbuf.clear();
               }
               ds.push(C::K(ck.clone()));
            }
         }
         if sbuf.len()>0 {
            ds.push(C::S(sbuf.clone()));
         }
         let mut dfa = match &ds[0] {
            C::S(s) => { compile_literal(&s) },
            C::K(k) => { compile_ast(&k) },
         };
         for di in 1..ds.len() {
         match &ds[di] {
            C::S(s) => { dfa = dfa.concatenate(&compile_literal(&s)) },
            C::K(k) => { dfa = dfa.concatenate(&compile_ast(&k)) },
         }}
         dfa
      },
      HirKind::Alternation(a) => {
         let mut dfa = compile_ast(a[0].kind());
         for ai in 1..a.len() {
            dfa = dfa.union(&compile_ast(a[ai].kind()));
         }
         dfa
      }
   }
}
