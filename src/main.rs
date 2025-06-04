use std::thread;
use std::time::Instant;

fn main() {
    let threads = num_cpus::get();
    println!("Number of CPU threads: {}", threads);

    addition(threads.clone());
    subtraction(threads.clone());
    multiplication(threads.clone());
    division(threads.clone());
    floating_point_operations(threads.clone());
    
}


fn addition(threads: usize) {
    let work_per_thread = 500_000_000; // Adjust for desired workload

    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            let mut sum = 0u64;
            for i in 0..work_per_thread {
                sum = sum.wrapping_add(i);
            }
            sum
        }));
    }

    let mut total = 0u64;
    for handle in handles {
        total = total.wrapping_add(handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("Total sum: {}", total);
    println!("Elapsed Add time: {:.2?}", duration);
}


fn subtraction(threads: usize) {
    let work_per_thread = 500_000_000; // Adjust for desired workload

    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            let mut sum = 0u64;
            for i in 0..work_per_thread {
                sum = sum.wrapping_sub(i);
            }
            sum
        }));
    }

    let mut total = 0u64;
    for handle in handles {
        total = total.wrapping_add(handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("Total subtraction: {}", total);
    println!("Elapsed Subtract time: {:.2?}", duration);
}

fn multiplication(threads: usize) {
    let work_per_thread = 500_000_000; // Adjust for desired workload

    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            let mut product = 1u64;
            for i in 1..=work_per_thread {
                product = product.wrapping_mul(i);
            }
            product
        }));
    }

    let mut total = 1u64;
    for handle in handles {
        total = total.wrapping_mul(handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("Total multiplication: {}", total);
    println!("Elapsed Multiply time: {:.2?}", duration);
}

fn division(threads: usize) {
    let work_per_thread = 500_000_000; // Adjust for desired workload

    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            let mut quotient: u64 = 1;
            for i in 1..=work_per_thread {
                if i != 0 {
                    quotient = quotient.wrapping_div(i);
                }
            }
            quotient
        }));
    }

    let mut total = 1u64;
    for handle in handles {
        total = total.wrapping_add(handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("Total division: {}", total);
    println!("Elapsed Divide time: {:.2?}", duration);
}

fn floating_point_operations(threads: usize) {
    let work_per_thread = 500_000_000; // Adjust for desired workload

    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            let mut sum = 0.0f64;
            for i in 0..work_per_thread {
                sum += (i as f64).sqrt();
            }
            sum
        }));
    }

    let mut total = 0.0f64;
    for handle in handles {
        total += handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Total floating point operations: {}", total);
    println!("Elapsed Floating Point time: {:.2?}", duration);
}

