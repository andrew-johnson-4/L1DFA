# L1 DFA
Deterministic Finite-State Automata Library for Rust, written in L1.

### Features

- `regex.try_parse()`
  - space complexity = $2^x$
  - time complexity = $2^x$
- `x.accepts(s)`
  - time complexity = $s$
- [`x.complement()`](https://cs.stackexchange.com/questions/49318/dfa-complement-dfa-on-a-b-that-accepts-string-where-the-numbers-of-as-n)
  - space complexity = $c(x)$
  - time complexity = $c(x)$
- [`x.intersect(y)`](https://math.stackexchange.com/questions/1166225/checking-understanding-of-dfa-regular-operations-intersection-and-star)
  - space complexity = $c(x)c(y)$
  - time complexity = $c(x)c(y)$
- [`x.minimize()`](https://en.wikipedia.org/wiki/DFA_minimization)
  - time complexity = $c(x)\log c(x)\Sigma_x$
- `x.is_subset_of(y)`
- [`x.reverse()`](https://cs.stackexchange.com/questions/39622/designing-a-dfa-and-the-reverse-of-it)
