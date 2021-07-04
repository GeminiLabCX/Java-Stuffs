from annoy import AnnoyIndex
import random
import sys

metrics = ["angular", "euclidean", "manhattan", "dot", "hamming"]
dim = int(sys.argv[1])
size = int(sys.argv[2])

for metric in metrics:
    fname = f'index.{metric}.{dim}d.ann'
    print(f'Generating index for {metric}')
    t = AnnoyIndex(dim, metric)  # Length of item vector that