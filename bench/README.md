# Benchmarks

+ Generate random index
  ```bash
  python3 builder.py
  ```
+ Run benckmarks
    ```bash
    # linux / MacOS
    ./bench.sh
    ```
    ```powershell
    # windows
    bench.bat
    ```

## Results (sample)

```verilog
[Python]annoy
angular Total time elapsed: 0.356s
angular Avg time elapsed: 0.356ms

[Python]annoy
euclidean Total time elapsed: 0.365s
euclidean Avg time elapsed: 0.365ms

[Rust] annoy-rs
[angular] Total time elapsed: 0.7767228s
[angular] Avg time elapsed: 0.7767228ms

[Rust] annoy-rs
[euclidean] Total time elapsed: 0.7305872s
[