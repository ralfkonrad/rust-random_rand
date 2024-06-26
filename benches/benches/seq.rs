// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]
#![allow(non_snake_case)]

extern crate test;

use test::Bencher;

use core::mem::size_of;
use rand::prelude::*;
use rand::seq::*;

// We force use of 32-bit RNG since seq code is optimised for use with 32-bit
// generators on all platforms.
use rand_pcg::Pcg32 as SmallRng;

const RAND_BENCH_N: u64 = 1000;

#[bench]
fn seq_shuffle_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng());
    let x: &mut [usize] = &mut [1; 100];
    b.iter(|| {
        x.shuffle(&mut rng);
        x[0]
    })
}

#[bench]
fn seq_slice_choose_1_of_1000(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng());
    let x: &mut [usize] = &mut [1; 1000];
    for (i, r) in x.iter_mut().enumerate() {
        *r = i;
    }
    b.iter(|| {
        let mut s = 0;
        for _ in 0..RAND_BENCH_N {
            s += x.choose(&mut rng).unwrap();
        }
        s
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

macro_rules! seq_slice_choose_multiple {
    ($name:ident, $amount:expr, $length:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = SmallRng::from_rng(thread_rng());
            let x: &[i32] = &[$amount; $length];
            let mut result = [0i32; $amount];
            b.iter(|| {
                // Collect full result to prevent unwanted shortcuts getting
                // first element (in case sample_indices returns an iterator).
                for (slot, sample) in result.iter_mut().zip(x.choose_multiple(&mut rng, $amount)) {
                    *slot = *sample;
                }
                result[$amount - 1]
            })
        }
    };
}

seq_slice_choose_multiple!(seq_slice_choose_multiple_1_of_1000, 1, 1000);
seq_slice_choose_multiple!(seq_slice_choose_multiple_950_of_1000, 950, 1000);
seq_slice_choose_multiple!(seq_slice_choose_multiple_10_of_100, 10, 100);
seq_slice_choose_multiple!(seq_slice_choose_multiple_90_of_100, 90, 100);

#[bench]
fn seq_iter_choose_multiple_10_of_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng());
    let x: &[usize] = &[1; 100];
    b.iter(|| x.iter().cloned().choose_multiple(&mut rng, 10))
}

#[bench]
fn seq_iter_choose_multiple_fill_10_of_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng());
    let x: &[usize] = &[1; 100];
    let mut buf = [0; 10];
    b.iter(|| x.iter().cloned().choose_multiple_fill(&mut rng, &mut buf))
}

macro_rules! sample_indices {
    ($name:ident, $fn:ident, $amount:expr, $length:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = SmallRng::from_rng(thread_rng());
            b.iter(|| index::$fn(&mut rng, $length, $amount))
        }
    };
}

sample_indices!(misc_sample_indices_1_of_1k, sample, 1, 1000);
sample_indices!(misc_sample_indices_10_of_1k, sample, 10, 1000);
sample_indices!(misc_sample_indices_100_of_1k, sample, 100, 1000);
sample_indices!(misc_sample_indices_100_of_1M, sample, 100, 1_000_000);
sample_indices!(misc_sample_indices_100_of_1G, sample, 100, 1_000_000_000);
sample_indices!(misc_sample_indices_200_of_1G, sample, 200, 1_000_000_000);
sample_indices!(misc_sample_indices_400_of_1G, sample, 400, 1_000_000_000);
sample_indices!(misc_sample_indices_600_of_1G, sample, 600, 1_000_000_000);

macro_rules! sample_indices_rand_weights {
    ($name:ident, $amount:expr, $length:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = SmallRng::from_rng(thread_rng());
            b.iter(|| {
                index::sample_weighted(&mut rng, $length, |idx| (1 + (idx % 100)) as u32, $amount)
            })
        }
    };
}

sample_indices_rand_weights!(misc_sample_weighted_indices_1_of_1k, 1, 1000);
sample_indices_rand_weights!(misc_sample_weighted_indices_10_of_1k, 10, 1000);
sample_indices_rand_weights!(misc_sample_weighted_indices_100_of_1k, 100, 1000);
sample_indices_rand_weights!(misc_sample_weighted_indices_100_of_1M, 100, 1_000_000);
sample_indices_rand_weights!(misc_sample_weighted_indices_200_of_1M, 200, 1_000_000);
sample_indices_rand_weights!(misc_sample_weighted_indices_400_of_1M, 400, 1_000_000);
sample_indices_rand_weights!(misc_sample_weighted_indices_600_of_1M, 600, 1_000_000);
sample_indices_rand_weights!(misc_sample_weighted_indices_1k_of_1M, 1000, 1_000_000);
