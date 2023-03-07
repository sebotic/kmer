## A Python 3 fast kmer counter module in Rust

For counting kmers in large datasets, speed is helpful. This litte fun 
Python 3 module relies on Rust for the heavy lifting, including
usage of multiple threads. 

As a convenient build tool, I used [maturin](https://www.maturin.rs/). So just [create a
virtual env](https://docs.python.org/3/library/venv.html) , install maturin and maturin just will install the
kmers module into your virtual env, like so.

```bash
$ git checkout https://github.com/sebotic/kmer.git
$ cd kmer
$ maturin develop
```

This is a rather fun, early project. At a later stage, 
I might create Python wheels and upload these to pypi. 