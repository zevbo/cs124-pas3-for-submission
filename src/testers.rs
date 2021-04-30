use core::num;
use rand::distributions::{Distribution, Uniform};
use std::{
    collections::{binary_heap::Iter, HashMap, HashSet},
    fmt::Display,
    iter::Sum,
    result,
};

use crate::signs;
use crate::{helpers, kk};
use rand::{random, Rng};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Algorithm {
    Random,
    Climbing,
    Annealing,
}

fn t(prev_result: f64, multiplier: f64) -> f64 {
    return prev_result * multiplier;
}
fn threshold(last_residue: i64, new_residue: i64, result: f64) -> f64 {
    assert!(new_residue > last_residue);
    let prob =
        // 2.718f64.powf(-(7. * (new_residue as f64 / last_residue as f64 - 1.)) as f64 / result);
        2.718f64.powf((last_residue - new_residue) as f64 / result);
    return prob;
}

fn general_test<T: IntoIterator>(
    a: &helpers::A,
    max_iter: i64,
    iters_to_track: &HashSet<i64>,
    algo: Algorithm,
    gen: fn(usize) -> T,
    eval: fn(&helpers::A, &T) -> i64,
    rand_edit: fn(&T) -> T,
) -> HashMap<i64, i64>
where
    T::Item: Display,
    T: Clone,
{
    let mut starting = gen(a.len());
    let mut last_residue = eval(a, &starting);
    let mut annealing_residue = last_residue;
    let mut rng = rand::thread_rng();
    let multiplier = 1.3f64.powf(1.0 / 300.);
    let mut prev_result = 10_i64.pow(10) as f64;
    let mut tracked_results = HashMap::new();
    tracked_results.insert(-1, kk::kk_evaluate(a));
    let distribution = Uniform::from(0.0..1.0);

    if iters_to_track.contains(&0) {
        tracked_results.insert(0, last_residue);
    }

    for i in 1..max_iter {
        let new_starting = match algo {
            Algorithm::Random => gen(a.len()),
            Algorithm::Climbing | Algorithm::Annealing => rand_edit(&starting),
        };
        let new_residue = eval(&a, &new_starting);
        prev_result = t(prev_result, multiplier);
        let use_new = new_residue <= last_residue
            || match algo {
                Algorithm::Annealing => {
                    distribution.sample(&mut rng)
                        < threshold(last_residue, new_residue, prev_result)
                }
                _ => false,
            };
        if use_new {
            starting = new_starting;
            last_residue = new_residue
        }
        match algo {
            Algorithm::Annealing => {
                if last_residue < annealing_residue {
                    annealing_residue = last_residue;
                }
            }
            _ => (),
        }
        if iters_to_track.contains(&i) {
            let residue = match algo {
                Algorithm::Annealing => annealing_residue,
                _ => last_residue,
            };
            tracked_results.insert(i, residue);
        }
    }
    return tracked_results;
}

pub fn partition_test(
    a: &helpers::A,
    max_iter: i64,
    iters_to_track: &HashSet<i64>,
    algo: Algorithm,
) -> HashMap<i64, i64> {
    return general_test(
        a,
        max_iter,
        iters_to_track,
        algo,
        kk::gen_partition,
        kk::evaluate,
        kk::rand_edit,
    );
}

pub fn signs_test(
    a: &helpers::A,
    max_iter: i64,
    iters_to_track: &HashSet<i64>,
    algo: Algorithm,
) -> HashMap<i64, i64> {
    return general_test(
        a,
        max_iter,
        iters_to_track,
        algo,
        signs::gen_signs,
        signs::evaluate,
        signs::rand_edit,
    );
}

// I know there's an O(n) implementation, but I don't think speed should matter here
// O(n log(n)) should be fine
fn median(vals: &Vec<&i64>, percentile: f64) -> f64 {
    let mut sorted = vals.clone();
    sorted.sort();
    let index = (percentile * (vals.len() as f64)) as usize;
    return *sorted[index] as f64;
}

#[derive(Copy, Clone)]
pub struct Summary {
    pub avg: f64,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
    pub min: i64,
    pub max: i64,
}

pub fn run_tests(
    num_tests: usize,
    size: usize,
    max_iter: i64,
    iters_to_track: HashSet<i64>,
    use_partitions: bool,
) -> HashMap<i64, (Summary, Summary, Summary)> {
    let mut all_results = HashMap::new();
    let algorithims = vec![Algorithm::Random, Algorithm::Climbing, Algorithm::Annealing];
    for algorithm in algorithims {
        all_results.insert(algorithm, Vec::<HashMap<i64, i64>>::new());
    }
    let test_f = if use_partitions {
        partition_test
    } else {
        signs_test
    };
    for run_on in 0..num_tests {
        let a = helpers::gen_a(size);
        println!("run_on: {}", run_on);
        for (algorithm, prev) in all_results.iter_mut() {
            prev.push(test_f(&a, max_iter, &iters_to_track, *algorithm));
        }
    }
    let mut all_data = HashMap::new();
    for iter in iters_to_track {
        let mut summaries = HashMap::new();
        for (algorithm, hashed_results) in all_results.iter() {
            let results = hashed_results
                .into_iter()
                .map(|hash| hash.get(&iter).unwrap());
            let results_v: Vec<&i64> = results.collect();
            let med = median(&results_v, 0.5);
            let q1 = median(&results_v, 0.25);
            let q3 = median(&results_v, 0.75);
            let min = **results_v.iter().min().unwrap();
            let max = **results_v.iter().max().unwrap();
            let avg = results_v.into_iter().sum::<i64>() as f64 / (num_tests as f64);
            let summary = Summary {
                avg,
                median: med,
                q1,
                q3,
                min,
                max,
            };
            summaries.insert(*algorithm, summary);
        }
        all_data.insert(
            iter,
            (
                *summaries.get(&Algorithm::Random).unwrap(),
                *summaries.get(&Algorithm::Climbing).unwrap(),
                *summaries.get(&Algorithm::Annealing).unwrap(),
            ),
        );
    }
    return all_data;
}
