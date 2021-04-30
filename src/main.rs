use std::{cmp::min, collections::HashSet, env, iter::Sum, print};

use kk::{kk_evaluate, rand_edit};
use std::fs;
use testers::{Algorithm, Summary};

mod helpers;
mod kk;
mod signs;
mod testers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (true) {
        let mut a = helpers::A::new();
        let filename = args.get(2).unwrap();
        let algorithim = args.get(1).unwrap();
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines = contents.split("\n");
        for num in lines.into_iter() {
            a.push((*num).parse().unwrap());
        }
        println!("algo: {}", algorithim);
        if algorithim == "0" {
            println!("{}", kk::kk_evaluate(&a));
        } else {
            let use_partitions = algorithim == "11" || algorithim == "12" || algorithim == "13";
            let algorithm = if algorithim == "1" || algorithim == "11" {
                testers::Algorithm::Random
            } else if algorithim == "2" || algorithim == "12" {
                testers::Algorithm::Climbing
            } else {
                testers::Algorithm::Annealing
            };
            let test_f = if use_partitions {
                testers::partition_test
            } else {
                testers::signs_test
            };
            let mut iters_to_check = HashSet::new();
            iters_to_check.insert(25000 - 1);
            let all_data_hash = test_f(&a, 25000, &iters_to_check, algorithm);
            println!("{}", all_data_hash.get(&24999).unwrap());
        }
    } else {
        println!("Hello, world!");
        let v = vec![1, 2, 2, 4, 0];
        for i in kk::rand_edit(&kk::rand_edit(&v)) {
            print!("{}, ", i);
        }
        let a = vec![10, 8, 7, 6, 5];
        assert!(kk::kk_evaluate(&a) == 2);
        let mut iters_to_track = HashSet::new();
        iters_to_track.insert(25000 - 1);
        let mut reworked_results: Vec<Vec<f64>> = vec![Vec::new(), Vec::new(), Vec::new()];
        let all_data_hash = testers::run_tests(100, 100, 25000, iters_to_track, true);
        type data = (i64, (Summary, Summary, Summary));
        fn print_summary(summ: &Summary) {
            println!("\nSummary:");
            println!("{}", summ.avg);
            println!("{}", summ.min);
            println!("{}", summ.q1);
            println!("{}", summ.median);
            println!("{}", summ.q3);
            println!("{}", summ.max);
        }

        let (rand_summary, climb_summary, anneal_summary) =
            all_data_hash.get(&(25000 - 1)).unwrap();
        print_summary(rand_summary);
        print_summary(climb_summary);
        print_summary(anneal_summary);

        /*
        let (rand_summary, climb_summary, anneal_summary) =
            all_data_hash.get(&(100000 - 1)).unwrap();
        print_summary(rand_summary);
        print_summary(climb_summary);
        print_summary(anneal_summary);
        */

        /*
        let mut all_data_v: Vec<data> = all_data_hash
            .into_iter()
            .map(|(iter, vals)| (iter, vals))
            .collect();
        all_data_v.sort_by(|(iter1, _data1), (iter2, _data2)| iter1.cmp(&iter2));
        for (_iter, (rand_summary, climb_summary, anneal_summary)) in all_data_v {
            reworked_results.get_mut(0).unwrap().push(rand_summary.avg);
            reworked_results.get_mut(1).unwrap().push(climb_summary.avg);
            reworked_results
                .get_mut(2)
                .unwrap()
                .push(anneal_summary.avg);
        }
        for v in reworked_results {
            println!("\n\n ---- NEW SECTION ----");
            for val in v {
                println!("{}", val);
            }
        }
        */
    }
}
