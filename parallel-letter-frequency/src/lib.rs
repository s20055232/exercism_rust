use std::collections::HashMap;
// use std::sync::mpsc::channel;
use std::thread;

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     assert!(worker_count > 0);
//     // 計算每個worker會需要處理的字串數量以及剩餘沒有被分配到的字串數量
//     let (jobs, rem) = (input.len() / worker_count, input.len() % worker_count);
//     // 如果有多餘的工作，每個vec size + 1，可以避免reallocate
//     let job_size = if rem > 0 { jobs + 1 } else { jobs };
//     // 幫每個worker建立專屬的工作清單
//     let mut parts: Vec<Vec<String>> = Vec::with_capacity(worker_count);
//     for _ in 0..worker_count {
//         parts.push(Vec::with_capacity(job_size));
//     }
//     // 依序將工作放進各個worker的工作清單，各放一次之後，重頭開始再放一輪，直到沒有工作
//     let mut i = 0;
//     for line in input.iter() {
//         // We'll need to clone those strings in order to satisfy some lifetime guarantees. Basically
//         // it's hard for the system to be sure that the threads spawned don't outlive the strings.
//         parts[i].push(line.to_string());
//         i = (i + 1) % worker_count;
//     }
//     // 建立通道，每個worker會執行工作，並將工作結果回傳回來做統整
//     let (tx, rx) = channel();

//     for part in parts {
//         let tx = tx.clone();
//         thread::spawn(move || {
//             tx.send(count(part)).unwrap();
//         });
//     }

//     let mut results: HashMap<char, usize> = HashMap::new();
//     for _ in 0..worker_count {
//         let part_results = rx.recv().unwrap();
//         for (c, n) in part_results.into_iter() {
//             *results.entry(c).or_insert(0) += n;
//         }
//     }
//     results
// }

// fn count(lines: Vec<String>) -> HashMap<char, usize> {
//     let mut results: HashMap<char, usize> = HashMap::new();
//     for line in lines.iter() {
//         for c in line.chars() {
//             if c.is_alphabetic() {
//                 *results.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
//             }
//         }
//     }
//     results
// }

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // 前：在thread0當中先將string都變成小寫，然後分配後threads各自運算
    // 後：分配到threads後再各自將string變成小寫，然後運算，速度比較快
    let worker_size = input.len() / worker_count + 1;
    let mut task_for_each: Vec<Vec<String>> = vec![Vec::with_capacity(worker_size); worker_count];
    let mut point = 0;
    for i in input {
        task_for_each[point].push(i.to_string());
        if point == worker_count - 1 {
            point = 0;
        } else {
            point += 1;
        }
    }
    let mut result: HashMap<char, usize> = HashMap::new();
    let mut handles = Vec::new();
    for i in task_for_each {
        let j = i.clone();
        let handle = thread::spawn(move || count(j));
        handles.push(handle);
    }
    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            result
                .entry(key)
                .and_modify(|counter| *counter += value)
                .or_insert(value);
        }
    }

    result
}

fn count(words: Vec<String>) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for i in words.iter() {
        for ch in i.to_lowercase().chars().filter(|x| x.is_alphabetic()) {
            counter
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    counter
}
