//! Various utilities for Dominion

use std::collections::VecDeque;

// Slightly modified from https://stackoverflow.com/questions/41208694/how-do-i-shuffle-a-vecdeque
// Requirement for Fisher-Yates shuffle: has a length and the ability to swap elements

/// Does the struct have a length and a function which swaps the location of two elements
pub trait LenAndSwap {
    fn len(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
    fn is_empty(&self) -> bool { self.len() == 0 }
}

/// Shuffles anything which implements the associated trait LenAndSwap
///
/// Uses Fisher-Yates shuffle
pub fn shuffle<T: LenAndSwap>(values: &mut T) {
    shuffle_with_rng(values, rand::thread_rng());
}

/// Shuffles an object which implements LenAndSwap given an rng
// Like rand::Rng::shuffle but for anything that implements LenAndSwap
fn shuffle_with_rng<T, R>(values: &mut T, mut rng: R)
    where T: LenAndSwap,
          R: rand::Rng {
    let mut i = values.len();
    while i >= 2 {
        // invariant: elements with index >= i have been locked in place.
        i -= 1;
        // lock element i in place.
        values.swap(i, rng.gen_range(0..i + 1));
    }
}

// Implement LenAndSwap for VecDeque
impl<T> LenAndSwap for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j)
    }
}
