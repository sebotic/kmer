import kmer
import numpy as np


def test_kmers():
    test_kmer = 'acgtacgtata'.upper()
    res, kmer_ind_map = kmer.calculate_kmers(test_kmer, 9)
    # print(res[:100])
    ind = kmer_ind_map.get('ACGTACGTA')

    print("kmer count for kmer 'ACGTACGTA' at index:", ind, "is", res[ind])
    assert np.sum(res) == 3
    assert 'ACGTACGTA' in kmer_ind_map

test_kmers()
