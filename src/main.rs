use std::sync::atomic::AtomicUsize;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

struct Message {}

fn main() {
    let (sender_a, receiver_a) = mpsc::channel::<Message>();
    let (sender_b, receiver_b) = mpsc::channel::<Message>();

    let atomic_counter = Arc::new(AtomicUsize::new(0));

    let atomic_counter_ping = atomic_counter.clone();
    let atomic_counter_monitor = atomic_counter.clone();

    let ping = thread::spawn(move || {
        let mut node = Message {};
        let mut counter = 0;
        loop {
            sender_a.send(node).expect("Ping: Failure sending");
            node = receiver_b.recv().expect("Ping: Failure receiving");
            counter += 1;
            atomic_counter_ping.store(counter, std::sync::atomic::Ordering::Relaxed);
        }
    });

    let pong = thread::spawn(move || loop {
        sender_b
            .send(receiver_a.recv().expect("Ping: Failure receiving"))
            .expect("Pong: Failure sending");
    });

    let monitor = thread::spawn(move || {
        let mut previous_value = 0;
        let mut previous_value_time = std::time::Instant::now();
        loop {
            thread::sleep(std::time::Duration::from_millis(1000));
            let current_value = atomic_counter_monitor.load(std::sync::atomic::Ordering::Acquire);
            if current_value != previous_value {
                let current_value_time = std::time::Instant::now();
                let millis_passed = (current_value_time
                    .duration_since(previous_value_time)
                    .as_millis()) as f32;
                let rate = (current_value - previous_value) as f32 / millis_passed;
                println!(
                    "PingPong rate per second: {}, millis passed {}",
                    1000 as f32 * rate,
                    millis_passed
                );
                previous_value = current_value;
                previous_value_time = current_value_time;
            }
        }
    });

    ping.join().expect("Failure joining ping");
    pong.join().expect("Failure joining pong");
    monitor.join().expect("Failure joining monitor");;

}
