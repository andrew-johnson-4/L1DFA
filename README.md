# L1 DFA
Deterministic Finite-State Automata Library for Rust, written in L1.

### Features

- [`try_parse(regex)`](https://www.cs.cornell.edu/courses/cs312/2004fa/lectures/rec26.htm)
  - space complexity = $O(2^x)$
  - time complexity = $O(2^x)$
  - note: most regexes aren't anywhere near this bad
  - this complexity comes from expressions like this `/(01|10)*[01]{x}/`
- `x.accepts(s)`
  - time complexity = $s$
- `x.is_empty()`
  - time complexity = $x$
- [`x.complement()`](https://cs.stackexchange.com/questions/49318/dfa-complement-dfa-on-a-b-that-accepts-string-where-the-numbers-of-as-n)
  - space complexity = $x$
  - time complexity = $x$
- [`x.intersect(y)`](https://math.stackexchange.com/questions/1166225/checking-understanding-of-dfa-regular-operations-intersection-and-star)
  - space complexity = $xy$
  - time complexity = $xy$
- [`x.minimize()`](https://en.wikipedia.org/wiki/DFA_minimization)
  - time complexity = $x\log x\Sigma_x$
- [`x.is_subset_of(y)`](https://cs.stackexchange.com/questions/9130/testing-whether-the-language-of-one-automaton-is-a-subset-of-another)
  - space complexity = $xy$
  - time complexity = $xy$
- [`x.reverse()`](https://cs.stackexchange.com/questions/39622/designing-a-dfa-and-the-reverse-of-it)
  - [space complexity](https://en.wikipedia.org/wiki/Powerset_construction) = $O(2^x)$
  - time complexity = $O(2^x)$
  - worst case example: `/[01]{x}(01|10)*/`
