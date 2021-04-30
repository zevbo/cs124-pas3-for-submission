use rand::Rng;

pub type A = Vec<i64>;

pub fn gen_unequal(len: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let ind1: usize = rng.gen::<usize>() % len;
    let ind2_temp: usize = rng.gen::<usize>() % (len - 1);
    let ind2 = ind2_temp + if ind2_temp >= ind1 { 1 } else { 0 };
    assert_ne!(ind1, ind2);
    return (ind1, ind2);
}

pub fn gen_a(len: usize) -> A {
    let max_num: i64 = 10_i64.pow(12);
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..len {
        v.push(rng.gen::<i64>().abs() % max_num);
    }
    return v;
}
