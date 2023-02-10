from annoy import AnnoyIndex
import random

metrics = ["angular", "euclidean", "manhattan","dot","hamming"]
dim = 5
size = 100

for metric in metrics:
    fname = f'index.{metric}.{dim}d.ann'
    print(f'Generating index for {metric}')
    # t = AnnoyInd