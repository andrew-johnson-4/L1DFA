//Manually Transpiled, TODO generate from L1 specification

pub struct DFA {
  pub states: Vec<bool>,
  pub transitions: std::collections::HashMap<(u64,char),u64>,
}

impl DFA {
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
    if lc == rc {
      let ps = ls * (right.states.len() as u64) + rs;
      let pt = lt * (right.states.len() as u64) + rt;
      p.transitions.insert((ps,*lc),pt);
    }}}
    p
  }

  pub fn complement(self: &DFA) -> DFA {
    let d = self;
    DFA {
      states: d.states.iter().map(|c| !c).collect::<Vec<bool>>(),
      transitions: self.transitions.clone(),
    }
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
}

use regex_syntax::Parser;
use regex_syntax::hir::HirKind;

pub fn try_parse(regex: &str) -> Option<DFA> {
   let Ok(hir) = Parser::new().parse(regex)
   else { return None; };
   let hir = hir.into_kind();
   try_compile(&hir)
}

pub fn try_compile(hir: &HirKind) -> Option<DFA> {
   match hir {
      HirKind::Empty => unimplemented!("try_compile Empty Regex"),
      HirKind::Literal(_l) => unimplemented!("try_compile Literal Regex"),
      HirKind::Class(_c) => unimplemented!("try_compile Class Regex"),
      HirKind::Anchor(_a) => unimplemented!("try_compile Anchor Regex"),
      HirKind::WordBoundary(_wb) => unimplemented!("try_compile Word Boundary Regex"),
      HirKind::Repetition(_r) => unimplemented!("try_compile Repetition Regex"),
      HirKind::Group(_g) => unimplemented!("try_compile Group Regex"),
      HirKind::Concat(_c) => unimplemented!("try_compile Concat Regex"),
      HirKind::Alternation(_a) => unimplemented!("try_compile Alternation Regex"),
   }
}
