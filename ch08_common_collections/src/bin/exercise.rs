use std::collections::HashMap;

fn main() {
    exercise1()
}

fn exercise1() {
    // 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
    // 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
    let mut numbers = vec![4, 4, 7, 9, 7, 1, 7, 2, 4, 3];
    let average = calculate_average(&numbers);
    let middle = calculate_middle(&mut numbers);
    let mode = calculate_mode(&numbers);
    println!("average: {}", average);
    println!("middle: {}", middle);
    println!("mode: {:?}", mode)
}

fn calculate_average(numbers: &[i32]) -> f64 {
    let total: i32 = numbers.iter().sum();
    total as f64 / numbers.len() as f64
}

fn calculate_middle(numbers: &mut [i32]) -> f64 {
    numbers.sort();
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f64 / 2.0
    } else {
        numbers[len / 2] as f64
    }
}

fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
    let mut counter: HashMap<i32, u32> = HashMap::new();
    for x in numbers {
        let count = counter.entry(*x).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode: Vec<i32> = Vec::new();
    for (k, v) in counter {
        if v > max_count {
            max_count = v;
            mode = vec![k];
        } else if v == max_count {
            mode.push(k);
        }
    }
    mode
}