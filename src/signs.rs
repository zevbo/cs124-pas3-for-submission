pub type SignsT = Vec<bool>;

use crate::helpers;
use rand::Rng;

pub fn gen_signs(len: usize) -> SignsT {
    let mut rng = rand::thread_rng();
    let mut v = SignsT::new();
    for _ in 0..len {
        v.push(rng.gen::<bool>());
    }
    return v;
}

pub fn evaluate(a: &helpers::A, signs: &SignsT) -> i64 {
    let zipped = a.iter().zip(signs.iter());
    let mut sum = 0;
    for (val, sign) in zipped {
        sum += (*val as i64) * (if *sign { 1 } else { -1 });
    }
    return sum.abs();
}

pub fn rand_edit(signs: &SignsT) -> SignsT {
    let mut new_signs = SignsT::clone(signs);
    let mut rng = rand::thread_rng();
    let (ind1, ind2) = helpers::gen_unequal(signs.len());
    new_signs[ind1] = !new_signs[ind1];
    if rng.gen::<bool>() {
        new_signs[ind2] = !new_signs[ind2];
    }
    return new_signs;
}
