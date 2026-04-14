use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Create 2 producer threads
    let mut producer_handles = vec![];

    for id in 0..2 {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        }));
    }

    // Create 3 consumer threads
    let mut consumer_handles = vec![];

    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signals (one per consumer)
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(0..100);

        println!("Producer {} generated {}", id, num);

        tx.send(num).unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }

        println!("Consumer {} processing {}", id, value);

        thread::sleep(Duration::from_millis(200));
    }

    println!("Consumer {} exiting", id);
}