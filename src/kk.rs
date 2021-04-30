use crate::helpers;
use rand::{prelude::ThreadRng, Rng};
use std::collections::BinaryHeap;

pub type PartitionT = Vec<usize>;

fn kk_pop(heap: &mut BinaryHeap<i64>) -> i64 {
    return match heap.pop() {
        None => 0,
        Some(val) => val,
    };
}

pub fn kk_evaluate(a: &helpers::A) -> i64 {
    let mut heap = BinaryHeap::new();
    for val in a {
        if *val != 0 {
            heap.push(*val);
        }
    }
    loop {
        let val1 = kk_pop(&mut heap);
        let val2 = kk_pop(&mut heap);
        if val2 == 0 {
            return val1;
        }
        heap.push(val1 - val2);
    }
}

pub fn apply_partition(a: &helpers::A, partition: &PartitionT) -> helpers::A {
    assert!(a.len() == partition.len());
    let mut new_a = vec![0; a.len()];
    for (i, group) in partition.iter().enumerate() {
        new_a[*group] += a[i];
    }
    return new_a;
}

pub fn evaluate(a: &helpers::A, partition: &PartitionT) -> i64 {
    return kk_evaluate(&apply_partition(a, partition));
}

fn gen_partition_group(len: usize, rng: &mut ThreadRng) -> usize {
    return (*rng).gen::<usize>() % len;
}

pub fn gen_partition(len: usize) -> PartitionT {
    let mut rng = rand::thread_rng();
    let mut v = PartitionT::new();
    for _ in 0..len {
        v.push(gen_partition_group(len, &mut rng));
    }
    return v;
}

pub fn rand_edit(partition: &PartitionT) -> PartitionT {
    let mut new_partition = PartitionT::clone(partition);
    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        let ind = rng.gen::<usize>() % partition.len();
        let mut new_group = new_partition[ind];
        while new_group == new_partition[ind] {
            new_group = gen_partition_group(partition.len(), &mut rng);
        }
        new_partition[ind] = new_group;
    }
    return new_partition;
}
