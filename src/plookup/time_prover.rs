use ark_ff::Field;


#[inline]
pub fn lookup<T: Copy>(v: &[T], index: &Vec<usize>) -> Vec<T> {
    index.iter().map(|&i| v[i]).collect()
}


#[inline]
fn alg_hash<F: Field>(v: &[F], w: &[F], chal: &F) -> Vec<F> {
    assert_eq!(v.len(), w.len());
    v.iter().zip(w).map(|(&v_i, &w_i) | v_i + w_i * chal).collect()
}

#[inline]
fn compute_lookup_vector_with_shift<F: Field>(v: &[F], gamma: &F, chi: &F, zeta: &F) -> Vec<F> {
    let mut res = Vec::new();
    let tmp = (F::one() + chi) * gamma;
    let mut prev = *v.last().unwrap() + F::from(v.len() as u64) * zeta;
    v.iter().enumerate().for_each(|(i, &e)| {
        let curr = e +  F::from(i as u64) * zeta;
        res.push(tmp + curr + prev * chi);
        prev = curr
    });
    res
}

#[inline]
pub fn plookup<F: Field>(
    subset: &[F],
    set: &[F],
    index_f: &[F],
    index: &[usize],
    gamma: &F,
    chi: &F,
    zeta: &F,
) -> (Vec<Vec<F>>, Vec<Vec<F>>, F, F, F, Vec<F>) {
    let mut lookup_vec = Vec::new();
    let mut accumulated_vec = Vec::new();

    // Compute the lookup vector for the subset
    let mut lookup_subset = Vec::new();
    let mut accumulated_subset = Vec::new();
    let mut tmp = F::one();
    subset.iter().zip(index_f.iter()).for_each(|(e, f)| {
        let x = *e + *zeta * f + gamma;
        lookup_subset.push(x);
        tmp *= x;
        accumulated_subset.push(tmp)
    });
    let lookup_subset_prod = *accumulated_subset.last().unwrap();
    lookup_vec.push(lookup_subset);
    accumulated_vec.push(accumulated_subset);

    // Compute the lookup vector for the set
    let lookup_set = compute_lookup_vector_with_shift(set, gamma, chi, zeta);
    let mut accumulated_set = Vec::new();
    let mut tmp = F::one();
    lookup_set.iter().for_each(|x| {
        tmp *= x;
        accumulated_set.push(tmp)
    });
    let lookup_set_prod = *accumulated_set.last().unwrap();
    lookup_vec.push(lookup_set);
    accumulated_vec.push(accumulated_set);

    // Compute the sorted vector
    let mut frequency = vec![1; set.len()];
    index.iter().for_each(|i| frequency[*i] += 1);
    let mut sorted = Vec::new();
    frequency
        .iter()
        .zip(set.iter())
        .for_each(|(f, e)| sorted.append(&mut vec![*e; *f]));

    // Compute the lookup vector for the sorted vector
    let lookup_sorted = compute_lookup_vector_with_shift(&sorted, gamma, chi, zeta);
    let mut accumulated_sorted = Vec::new();
    let mut tmp = F::one();
    lookup_sorted.iter().for_each(|x| {
        tmp *= x;
        accumulated_sorted.push(tmp)
    });
    let lookup_sorted_prod = *lookup_sorted.last().unwrap();
    lookup_vec.push(lookup_sorted);
    accumulated_vec.push(accumulated_sorted);

    (
        lookup_vec,
        accumulated_vec,
        lookup_subset_prod,
        lookup_set_prod,
        lookup_sorted_prod,
        sorted,
    )
}