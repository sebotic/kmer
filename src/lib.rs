use hashbrown::{HashMap, HashSet};
use pyo3::prelude::*;
use rayon::prelude::*;

#[pymodule]
fn kmer(_py: Python, m: &PyModule) -> PyResult<()> {
    println!("Successfully loaded kmer Rust module");

    #[pyfn(m)]
    fn calculate_kmers(_py: Python, l: &str, length: usize) -> PyResult<Vec<usize>> {
        let out = gen_kmers(&l, length);
        Ok(out)
    }

    Ok(())
}

// const CHARACTERS: Vec<&str> = vec!["a", "c", "g", "t"];
// const KMER_LEN: u32 = 9;
// static KMER_INDEX_MAP: HashMap<&str, i32> = (2..KMER_LEN).fold(
//     ("a", "c", "g", "t").iter().map(|c| ("a", "c", "g", "t").iter().map(move |&d| d.to_owned() + *c)).flatten().collect(),
//     |acc,_| acc.into_iter().map(|c| ("a", "c", "g", "t").iter().map(move |&d| d.to_owned() + &*c)).flatten().collect())
//     .map(|c| (c, 0_u32)).collect::<HashMap<_, _>>();

fn gen_kmers(msa: &str, kmer_len: usize) -> Vec<usize> {
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
    // println!("{:?}", combinations);

    // let filename = "5343819_128Taxa_GTR+G__alpha2__100000_59873.phy";
    //
    // let contents: String = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");

    // let s1 = vec!["acgtacgttg", "atttaaaaaaaaa", "acgttccttttt", "acgtccccccc"];
    let lines: Vec<&str> = msa
        .lines()
        .filter(|x| x.len() > 0)
        .map(|c| c.split(" ").last().unwrap())
        .collect();

    // println!("number of taxa: {:?}", lines.len());
    // println!("{:?}", lines.first().unwrap());

    // let mut hmap: HashMap<&str, u32> = HashMap::new();
    // for (c, x) in combinations.iter().enumerate(){
    //     hmap.insert(x, c as u32);
    // }

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
    // println!("Length of sequences: {:?}", res);

    let mut counts: Vec<_> = vec![0 as usize; combinations.len()];

    // (0..res.len()).map(|x| res[x].1.iter().map(|&c | {counts[c] += 1; c})).collect::<Vec<usize>>();
    for x in res.iter() {
        for k in x.1.iter() {
            counts[*k] += 1;
        }
    }

    counts
}
