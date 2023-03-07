import kmer
import numpy as np


def test_kmers():
    test_kmer = 'acgtacgtata'.upper()
    res = np.array(kmer.calculate_kmers(test_kmer, 9))
    assert np.sum(res) == 3



