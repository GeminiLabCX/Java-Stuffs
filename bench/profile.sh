#!/bin/bash

# python3 -m pip install -U wheel annoy

# python3 builder.py 256 10000

python3 bencher.py 256 10000 200 1000
valgrind --tool=callgrind --callgrind-out-file=callgrind.nosimd.out -- pytho