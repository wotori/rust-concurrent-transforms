use std::sync::Arc;
use std::thread;

const THRESHOLD: usize = 100;

fn process_data<T, F, R>(input: Vec<T>, func: F) -> Vec<R>
where
    T: Send + Sync + 'static + Clone,
    R: Send + 'static,
    F: Fn(T) -> R + Sync + Send + 'static,
{
    let input_len = input.len();
    if input_len <= THRESHOLD {
        input.into_iter().map(func).collect()
    } else {
        let input = Arc::new(input);
        let func = Arc::new(func);

        let num_threads =
            std::thread::available_parallelism().map_or(4, |n| n.get().min(input_len));

        let chunk_size = input_len / num_threads;
        let mut threads = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            let input = Arc::clone(&input);
            let func = Arc::clone(&func);
            let handle = thread::spawn(move || {
                let start = i * chunk_size;
                let end = if i == (num_threads - 1) {
                    input_len
                } else {
                    (i + 1) * chunk_size
                };
                input[start..end]
                    .iter()
                    .cloned()
                    .map(|item| (func)(item))
                    .collect::<Vec<R>>()
            });
            threads.push(handle);
        }

        let mut result = Vec::with_capacity(input_len);
        for handle in threads {
            result.extend(handle.join().unwrap());
        }
        result
    }
}

fn transform_number(mut n: u64, max_iter: usize) -> u64 {
    let mut count = 0;
    while n != 1 && count < max_iter {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        count += 1;
    }
    if n == 1 {
        count as u64
    } else {
        n
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 100];
    let k = 8;
    let results = process_data(numbers, move |x| transform_number(x, k));
    println!("{:?}", results); // Expected result: [0, 1, 7, 88]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_thread_processing() {
        assert_eq!(process_data(vec![1, 2, 3], |x| x + 1), vec![2, 3, 4]);
    }

    #[test]
    fn test_multi_thread_processing() {
        let large_input = (0..100000).collect::<Vec<_>>();
        assert_eq!(
            process_data(large_input.clone(), |x| x + 1),
            large_input.iter().map(|&x| x + 1).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_transform_number_to_one() {
        assert_eq!(transform_number(1, 10), 0);
        assert_eq!(transform_number(2, 10), 1);
    }

    #[test]
    fn test_transform_number_max_iter() {
        assert_eq!(transform_number(3, 2), 5);
    }
}
