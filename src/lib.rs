//use hashbrown::{HashMap, HashSet};
use std::collections::HashMap;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pymodule]
fn kmer(_py: Python, m: &PyModule) -> PyResult<()> {
    println!("Successfully loaded kmer Rust module");

    #[pyfn(m)]
    fn calculate_kmers(_py: Python, l: &str, length: usize) -> PyResult<(Vec<usize>, HashMap<String, usize>)> {
        let out = gen_kmers(&l, length);
        Ok(out)
    }

    Ok(())
}

fn gen_kmers(msa: &str, kmer_len: usize) -> (Vec<usize>, HashMap<String, usize>) {
    let characters: Vec<&str> = vec!["A", "C", "G", "T"];

    let combinations: Vec<_> = (2..kmer_len).fold(
        characters
            .iter()
            .map(|c| characters.iter().map(move |&d| d.to_owned() + *c))
            .flatten()
            .collect(),
        |acc, _| {
            acc.into_iter()
                .map(|c| characters.iter().map(move |&d| d.to_owned() + &*c))
                .flatten()
                .collect()
        },
    );

    let kmer_index_map = combinations
        .iter()
        .enumerate()
        .map(|(count, kmer)| (kmer.as_str(), count as usize))
        .collect::<HashMap<_, _>>();

    println!("combination length {}", combinations.len());

    let lines: Vec<&str> = msa
        .lines()
        .filter(|x| x.len() > 0)
        .map(|c| c.split(" ").last().unwrap())
        .collect();

    let res: Vec<_> = lines
        .par_iter()
        .enumerate()
        .map(|(c, y)| {
            let m: Vec<usize> = (0..lines[c].len() - kmer_len + 1)
                .map(|cc| *kmer_index_map.get(&y[cc..cc + kmer_len]).unwrap())
                .collect::<Vec<usize>>();
            (c, m)
        })
        .collect();

    let mut counts: Vec<_> = vec![0 as usize; combinations.len()];

    for x in res.iter() {
        for k in x.1.iter() {
            counts[*k] += 1;
        }
    }

    (counts, kmer_index_map.into_iter().map(|(k, v)| (String::from(k), v) ).collect::<HashMap<String, usize>>())
}
