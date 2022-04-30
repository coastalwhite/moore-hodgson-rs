//! An implementation of the Moore-Hodgson algorithm within `no_std` rust.
//!
//! The [Moore-Hodgson algorithm](https://www.bsraya.com/portfolio/moore-hodgson-algorithm/) is a
//! scheduling algorithm that minimizes the amount of late jobs.
//!
//! It provides a single function [`moore_hodgson`], that performs the algorithm.
//!
//! # Examples
//!
//! ```
//! use moore_hodgson::moore_hodgson;
//!
//! let mut jobs = [
//!     ("ApplyForJob",        6, 5), // Due after 6 time units, takes 5 time units
//!     ("FileTaxes",          7, 1), // Due after 7 time units, takes 1 time unit
//!     ("BuyPresentForMom",   4, 1), // Due after 4 time units, takes 1 time unit
//!     ("SolveUrgentProblem", 6, 4), // Due after 6 time units, takes 4 time units
//!     ("ApplyForLoan",       8, 3), // Due after 8 time units, takes 3 time units
//! ];
//!
//! let nr_of_on_time_jobs = moore_hodgson(&mut jobs);
//! 
//! assert_eq!(nr_of_on_time_jobs, 3);
//! // jobs = [
//! # assert_eq!(jobs[0].0, BuyPresentForMom);
//! //    (BuyPresentForMom,   4, 1),
//! # assert_eq!(jobs[1].0, ApplyForJob);
//! //    (ApplyForJob,        6, 5),
//! # assert_eq!(jobs[2].0, FileTaxes);
//! //    (FileTaxes,          7, 1),
//! # assert_eq!(jobs[3].0, ApplyForLoan);
//! //    (ApplyForLoan,       8, 3),
//! # assert_eq!(jobs[4].0, SolveUrgentProblem);
//! //    (SolveUrgentProblem, 6, 4),
//! // ]
//! ```
//!
//! # License
//!
//! Licensed under a __MIT__ license.

#![no_std]
#![warn(missing_docs)]

use core::ops::Add;

/// Run the Moore-Hudgson's Algorithm on the array with items of form `(item, due_time,
/// processing_time)`. Returns the amount of items that will be on time.
///
/// Note that this function always move items with a `due_time` that cannot be compared to the
/// beginning of the late items. This happens for [`f32::NAN`], for example.
///
/// # Complexity
/// This function runs in `O(n^2)` time.
pub fn moore_hodgson<T, D, P>(items: &mut [(T, D, P)]) -> usize
where
    D: Clone + PartialOrd,
    P: Clone + Add<P, Output = P> + Default + PartialOrd<D>,
{
    let mut on_time_items_end = 0;
    let mut late_items_start = items.len();

    let mut completion_time = P::default();

    // While all the items are processed
    while on_time_items_end != late_items_start {
        // Find the minimum item
        let mut min_index = on_time_items_end;
        let mut min_due_time = &items[min_index].1;
        let mut min_processing_time = &items[min_index].2;
        for i in (on_time_items_end + 1)..late_items_start {
            let (_, ref due_time, ref processing_time) = &items[i];

            if due_time < min_due_time {
                min_index = i;
                min_due_time = due_time;
                min_processing_time = processing_time;
            }
        }

        // Move the minimum item to the correct spot
        let end_time = completion_time.clone() + min_processing_time.clone();

        if end_time <= min_due_time.clone() {
            items.swap(min_index, on_time_items_end);
            on_time_items_end += 1;
            completion_time = end_time;
        } else {
            late_items_start -= 1;
            items.swap(min_index, late_items_start);
        }
    }

    on_time_items_end
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tst_combination {
        ($input:expr, $output:expr, $on_time_items:expr) => {
            let mut scheduled_jobs = $input.clone();

            assert_eq!(moore_hodgson(&mut scheduled_jobs), $on_time_items);

            assert_eq!(
                scheduled_jobs.len(),
                $input.len(),
                "Moore-Hudgson changed size of array"
            );
            for i in 0..$output.len() {
                assert_eq!(scheduled_jobs[i], $output[i], "Item {} is not equal", i);
            }
        };
    }

    #[test]
    fn bsraya_examples() {
        // https://www.bsraya.com/portfolio/moore-hodgson-algorithm/
        let jobs = [(1, 6, 4), (2, 7, 3), (5, 11, 6), (4, 9, 5), (3, 8, 2)];

        tst_combination!(jobs, jobs, 2);
        tst_combination!(
            [
                (1, 6, 4),
                (2, 8, 1),
                (3, 9, 6),
                (4, 11, 3),
                (5, 20, 6),
                (7, 25, 8),
                (6, 28, 7),
                (8, 35, 10)
            ],
            [
                (1, 6, 4),
                (2, 8, 1),
                (4, 11, 3),
                (5, 20, 6),
                (7, 25, 8),
                (8, 35, 10),
                (6, 28, 7),
                (3, 9, 6),
            ], 6
        );
    }

    #[test]
    fn bsraya_unordered() {
        tst_combination!(
            [
                (6, 28, 7),
                (1, 6, 4),
                (7, 25, 8),
                (2, 8, 1),
                (5, 20, 6),
                (3, 9, 6),
                (8, 35, 10),
                (4, 11, 3),
            ],
            [
                (1, 6, 4),
                (2, 8, 1),
                (4, 11, 3),
                (5, 20, 6),
                (7, 25, 8),
                (8, 35, 10),
                (6, 28, 7),
                (3, 9, 6),
            ], 6
        );
        tst_combination!(
            [
                (4, 11, 3),
                (5, 20, 6),
                (7, 25, 8),
                (1, 6, 4),
                (6, 28, 7),
                (3, 9, 6),
                (8, 35, 10),
                (2, 8, 1),
            ],
            [
                (1, 6, 4),
                (2, 8, 1),
                (4, 11, 3),
                (5, 20, 6),
                (7, 25, 8),
                (8, 35, 10),
                (6, 28, 7),
                (3, 9, 6),
            ], 6
        );
    }

    #[test]
    fn nan_in_after() {
        let empty_output: [(usize, f32, f32); 0] = [];
        tst_combination!([(1, f32::NAN, 3.)], empty_output, 0);
        tst_combination!([(1, f32::NAN, 3.), (2, 7., 6.)], [(2, 7., 6.)], 1);
    }

    #[test]
    fn infinity() {
        let empty_output: [(usize, f32, f32); 0] = [];
        tst_combination!([(1, f32::INFINITY, 3.)], empty_output, 1);
        tst_combination!([(1, f32::NEG_INFINITY, 0.)], empty_output, 0);
    }

    #[test]
    fn zero_due_time() {
        tst_combination!([(1, 0, 3)], [(1, 0, 3)], 0);
        tst_combination!([(1, 0, 0)], [(1, 0, 0)], 1);
        tst_combination!([(1, 0, 0), (2, 5, 5), (3, 6, 2)], [(1, 0, 0), (2, 5, 5), (3, 6, 2)], 2);
    }
    #[test]
    fn zero_processing_time() {
        tst_combination!([(1, 5, 0)], [(1, 5, 0)], 1);
        tst_combination!([(1, 5, 0), (2, 5, 5)], [(1, 5, 0), (2, 5, 5)], 2);
        tst_combination!([(1, 0, 0), (2, 5, 5), (3, 6, 2)], [(1, 0, 0), (2, 5, 5), (3, 6, 2)], 2);
    }
}
