from annoy import AnnoyIndex
import random
import sys

metrics = ["angular", "euclidean", "manhattan", "dot", "hamming"]
dim = int(sys.argv[1])
size = int(sys.argv[2])

for metric in metrics:
    fname = f