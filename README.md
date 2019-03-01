# rs_pingpong
PingPong written in Rust

# What is this?

An excuse to start learning Rust. 2 threads exchanging numbers by AtomicUsize and mpsc channels.

# How to run it 

1. Clone repo
2. ```cargo build --release```
3. ```target/release/rs_pingpong```

## Sample output

```bash
target/release/rs_pingpong
PingPong[strategy: AtomicUsize, rate per second: 7824726.13].
PingPong[strategy: AtomicUsize, rate per second: 7727975.27].
PingPong[strategy: AtomicUsize, rate per second: 7849293.56].
PingPong[strategy: AtomicUsize, rate per second: 7692307.69].
PingPong[strategy: AtomicUsize, rate per second: 8196721.31].
PingPong[strategy: AtomicUsize, rate per second: 14792899.41].
PingPong[strategy: AtomicUsize, rate per second: 7782101.17].
PingPong[strategy: AtomicUsize, rate per second: 16666666.67].
PingPong[strategy: AtomicUsize, rate per second: 7710100.23].
PingPong[strategy: AtomicUsize, rate per second: 7830853.56].
PingPong[strategy: AtomicUsize, rate per second: 7886435.33].
PingPong[strategy: AtomicUsize, rate per second: 8012820.51].
PingPong[strategy: AtomicUsize, rate per second: 23094688.22].
PingPong[strategy: AtomicUsize, rate per second: 7710100.23].
PingPong[strategy: AtomicUsize, rate per second: 22935779.82].
PingPong[strategy: AtomicUsize, rate per second: 7710100.23].
PingPong[strategy: AtomicUsize, rate per second: 22779043.28].
PingPong[strategy: AtomicUsize, rate per second: 7874015.75].
PingPong[strategy: AtomicUsize, rate per second: 23094688.22].
PingPong[strategy: mpsc channel, rate per second: 3318951.21].
PingPong[strategy: mpsc channel, rate per second: 2797202.80].
PingPong[strategy: mpsc channel, rate per second: 9784735.81].
PingPong[strategy: mpsc channel, rate per second: 2590673.58].
PingPong[strategy: mpsc channel, rate per second: 2692514.81].
PingPong[strategy: mpsc channel, rate per second: 2656042.50].
PingPong[strategy: mpsc channel, rate per second: 2748763.06].
PingPong[strategy: mpsc channel, rate per second: 4638218.92].
PingPong[strategy: mpsc channel, rate per second: 3473428.27].
PingPong[strategy: mpsc channel, rate per second: 2581977.79].
PingPong[strategy: mpsc channel, rate per second: 2707825.62].
PingPong[strategy: mpsc channel, rate per second: 2610284.52].
PingPong[strategy: mpsc channel, rate per second: 3170577.05].
PingPong[strategy: mpsc channel, rate per second: 2559508.57].
PingPong[strategy: mpsc channel, rate per second: 2734481.82].
PingPong[strategy: mpsc channel, rate per second: 2624671.92].
PingPong[strategy: mpsc channel, rate per second: 2613012.80].
PingPong[strategy: mpsc channel, rate per second: 2965599.05].
PingPong[strategy: mpsc channel, rate per second: 2647603.92].
```
