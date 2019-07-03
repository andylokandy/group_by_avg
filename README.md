# Group By Avg

A simple quiz that groups intergers by keys and aggregates their values by `avg()`.

## Usage

To  verify :

```rust
cargo test
```

To run benchmark :

```rust
cargo bench
```

To test it on a concrete data file :

```rust
cargo run --example from_file
```

## Benchmark

First version :

```text
normal 10000            time:   [730.72 us 736.59 us 743.19 us]
                        change: [-0.6531% +0.3187% +1.3352%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

dense key 10000         time:   [709.41 us 714.15 us 720.16 us]
                        change: [-1.6912% -0.2163% +0.9607%] (p = 0.78 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

sparse key 10000        time:   [1.0684 ms 1.0763 ms 1.0856 ms]
                        change: [+0.5442% +1.8321% +3.3097%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```

Improved version :

```text
normal 10000            time:   [616.49 us 638.18 us 661.64 us]
                        change: [-17.586% -14.066% -9.3546%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

dense key 10000         time:   [613.46 us 629.98 us 647.16 us]
                        change: [-14.804% -10.404% -4.2484%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

sparse key 10000        time:   [686.66 us 689.32 us 692.22 us]
                        change: [-37.499% -36.630% -35.859%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
```
