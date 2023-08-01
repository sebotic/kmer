## A Python fast kmer counter module in Rust

For counting kmers in large bioinformatics DNA datasets, speed is helpful. This litte fun 
Python 3 module relies on Rust for the heavy lifting, including
usage of multiple threads. 

As a convenient build tool, I used [maturin](https://www.maturin.rs/). So just [create a
virtual env](https://docs.python.org/3/library/venv.html) , install maturin and maturin will install the
kmers module into your virtual env, like so:

```bash
$ git clone https://github.com/sebotic/kmer.git
$ cd kmer
$ maturin develop
```

This is a rather fun, early project. At a later stage, 
I might create Python wheels and upload these to pypi. 

### Usage
Currently, there's only a single function which takes 
a string and the kmer length and returns a tuple containing a list of 
kmer counts as well as a dictionary mapping kmers 
to count list indices.

Example, for 9mers:

```Python
import kmer
kmer_counts, kmer_indices = kmer.calculate_kmers("AGCTCCCGTAG", 9)
```
