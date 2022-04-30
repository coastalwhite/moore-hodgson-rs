# Moore Hodgson in Rust
![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![moore-hodgson on crates.io](https://img.shields.io/crates/v/moore-hodgson)](https://crates.io/crates/moore-hodgson) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/coastalwhite/moore-hodgson)

An implementation of the Moore-Hudgson algorithm within `no_std` rust.

The [Moore-Hudgson algorithm][__link0] is a scheduling algorithm that minimizes the amount of late jobs.

It provides a single function [`moore_hodgson`][__link1], that performs the algorithm.


## Examples


```rust
use moore_hodgson::moore_hodgson;

let mut jobs = [
    ("ApplyForJob",        6, 5), // Due after 6 time units, takes 5 time units
    ("FileTaxes",          7, 1), // Due after 7 time units, takes 1 time unit
    ("BuyPresentForMom",   4, 1), // Due after 4 time units, takes 1 time unit
    ("SolveUrgentProblem", 6, 4), // Due after 6 time units, takes 4 time units
    ("ApplyForLoan",       8, 3), // Due after 8 time units, takes 3 time units
];

let nr_of_on_time_jobs = moore_hodgson(&mut jobs);

assert_eq!(nr_of_on_time_jobs, 3);
// jobs = [
//    (BuyPresentForMom,   4, 1),
//    (ApplyForJob,        6, 5),
//    (FileTaxes,          7, 1),
//    (ApplyForLoan,       8, 3),
//    (SolveUrgentProblem, 6, 4),
// ]
```


## License

Licensed under a **MIT** license.


 [__link0]: https://www.bsraya.com/portfolio/moore-hodgson-algorithm/
 [__link1]: https://docs.rs/moore-hodgson/0.1.0/moore_hodgson/?search=moore_hodgson::moore_hodgson
