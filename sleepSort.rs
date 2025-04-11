use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut numbers = [83, 12, 13, 35, 91, 71, 75, 58, 26, 38, 2, 23, 10];
    sleep_sort(&mut numbers);
    assert_eq!(numbers, [2, 10, 12, 13, 23, 26, 35, 38, 58, 71, 75, 83, 91]);
}

fn sleep_sort(arr: &mut [u64]) {
    let mut multiplier = 1.0; // Factor to scale sleep duration
    let mut retry_count = 0; // Track sorting attempts

    loop {
        // Shared vector to collect sorted elements
        let shared_result = Arc::new(Mutex::new(Vec::new()));
        let mut thread_pool = Vec::new();

        // Spawn threads for each element in the array
        for &value in arr.iter() {
            let shared_result_clone = Arc::clone(&shared_result);

            thread_pool.push(thread::spawn(move || {
                let sleep_time = (value as f64 * multiplier) as u32;
                thread::sleep(Duration::new(0, sleep_time));
                shared_result_clone.lock().unwrap().push(value);
            }));
        }

        // Wait for all threads to complete execution
        for thread in thread_pool {
            thread.join().unwrap();
        }

        retry_count += 1;
        multiplier *= 2.0; // Adjust multiplier for next attempt

        // Extract sorted results from the shared vector
        let sorted_result = Arc::try_unwrap(shared_result)
            .expect("Failed to unwrap Arc")
            .into_inner()
            .expect("Failed to unlock Mutex");

        // Check if the array is sorted; if yes, update the original array
        if sorted_result.windows(2).all(|w| w[0] <= w[1]) {
            arr.copy_from_slice(&sorted_result);
            break;
        }
    }

    println!("Sorting completed after {} attempts (Multiplier: {})", retry_count, multiplier);
}
