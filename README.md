# L1 DFA
Deterministic Finite-State Automata Library for Rust, written in L1.

### Features

- `regex.try_parse()`
- `x.accepts(s)`
- `x.complement()`
- [`x.intersect(y)`](https://math.stackexchange.com/questions/1166225/checking-understanding-of-dfa-regular-operations-intersection-and-star)
  - space complexity = $c(L_1)c(L_2)$
  - time complexity = $ns\log n$
- [`x.minimize()`](https://en.wikipedia.org/wiki/DFA_minimization)
- `x.is_subset_of(y)`
- `x.reverse()`
