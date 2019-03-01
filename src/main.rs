use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn ping_pong_atomic(
    ping_pongs: usize,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>) {
    let atomic_counter = Arc::new(AtomicUsize::new(2));

    let atomic_counter_ping = atomic_counter.clone();
    let atomic_counter_pong = atomic_counter.clone();

    let ping = thread::spawn(move || {
        for _i in 1..ping_pongs {
            atomic_counter_ping.store(1, Ordering::SeqCst);
            let mut read_value = atomic_counter_ping.load(Ordering::SeqCst);
            while read_value == 1 {
                read_value = atomic_counter_ping.load(Ordering::SeqCst);
            }
        }
    });

    let pong = thread::spawn(move || {
        for _i in 1..ping_pongs {
            let mut read_value = atomic_counter_pong.load(Ordering::SeqCst);
            while read_value == 2 {
                read_value = atomic_counter_pong.load(Ordering::SeqCst);
            }
            atomic_counter_pong.store(2, Ordering::SeqCst);
        }
    });

    (ping, pong)
}

fn ping_pong_channel(
    ping_pongs: usize,
) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>) {
    let (sender_a, receiver_a) = mpsc::channel::<usize>();
    let (sender_b, receiver_b) = mpsc::channel::<usize>();

    let ping = thread::spawn(move || {
        for _i in 1..ping_pongs {
            sender_a.send(1).expect("Ping: Failure sending");
            loop {
                match receiver_b.try_recv() {
                    Ok(_value) => break,
                    Err(mpsc::TryRecvError::Empty) => continue,
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Ping: Failure receiving"),
                }
            }
        }
    });

    let pong = thread::spawn(move || {
        for _i in 1..ping_pongs {
            loop {
                match receiver_a.try_recv() {
                    Ok(_value) => break,
                    Err(mpsc::TryRecvError::Empty) => continue,
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Pong: Failure receiving"),
                }
            }
            sender_b.send(2).expect("Pong: Failure sending");
        }
    });

    (ping, pong)
}

fn measure<F: FnMut() -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)>(
    strategy: &str,
    iterations: usize,
    ping_pongs: usize,
    mut ping_pong: F,
) {
    for _i in 1..iterations {
        let start = Instant::now();
        let (ping, pong) = ping_pong();
        ping.join().expect("Failure joining Ping thread");
        pong.join().expect("Failure joining Pong thread");
        let elapsed = Instant::now().duration_since(start);

        println!(
            "PingPong[strategy: {}, rate per second: {:.2}].",
            strategy,
            1000 as f64 * ping_pongs as f64 / elapsed.as_millis() as f64
        );
    }
}

fn main() {
    const PING_PONGS: usize = 10000000;
    const ITERATIONS: usize = 20;

    measure("AtomicUsize", ITERATIONS, PING_PONGS, || {
        ping_pong_atomic(PING_PONGS)
    });

    measure("mpsc channel", ITERATIONS, PING_PONGS, || {
        ping_pong_channel(PING_PONGS)
    });
}
