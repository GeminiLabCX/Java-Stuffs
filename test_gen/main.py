from annoy import AnnoyIndex
import random

metrics = ["angular", "euclidean", "manhattan","dot","hamming"]
dim = 5
size = 100

for metric in metrics:
    fname = f'index.{metric}.{dim}d.ann'
    print(f'Generating index for {metric}')
    # t = AnnoyIndex(dim, metric)  # Length of item vector that will be indexed
    # for i in range(size):
    #     v = [random.gauss(0, 1) for z in range(dim)]
    #     t.add_item(i, v)

    # t.build(10)  # 10 trees
    # t.save(fname)

    # ...

    u = AnnoyIndex(dim, metric)
    u.verbose(True)
    u.load('./../tests/'+fname)  # supe