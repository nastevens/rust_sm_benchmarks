# Rust State Machine Benchmarks

Benchmarks testing different state machine implementations

There are many different ways to implement state machines in Rust.  This module
collects 3 of them and an compares their performance.

Here are the benchmark results from my machine (Macbook Pro 2.5Ghz i7).
Descriptions of the implementations are below.

```
test tests::bench_boxed       ... bench:  23,237,914 ns/iter (+/- 3,424,072)
test tests::bench_mem_replace ... bench:     606,998 ns/iter (+/- 112,135)
test tests::bench_static_ref  ... bench:   3,381,536 ns/iter (+/- 491,512)
```


## Boxed state machine

This is the simplest state machine to reason about, and the most recommended on
help sites. Each event handler returns the next state as a boxed trait, and
dispatch is done dynamically.

Pros:
  - Lowest boilerplate
  - Very easy to reason about
  - Open to addition of new states

Cons:
  - Least performant by an order of magnitude


## `mem::replace` enum

Models all states as their own objects, allowing unique variables to be
associated with each state. All states are then collected in an enum wrapper
type, with event functions returning a new state + enum wrapper.  At the top
level, the old state is replaced with the new state using `mem::replace`.

Pros:
  - Best performance

Cons:
  - Requires extra "dispatch" step for each event type
  - Extra states must be added to module-wide enum
  - Return types from handler functions must be wrapped with enum type


## Static references to states

Models all states as unit structs and switches based on static references to
those structs. All state variables must be kept external and passed to each
function.

Pros:
  - Low boilerplate
  - Open to addition of new states

Cons:
  - All state variables must be kept external to state objects


# License

This code is licensed under the MIT license. See `LICENSE.txt` for full
information.
