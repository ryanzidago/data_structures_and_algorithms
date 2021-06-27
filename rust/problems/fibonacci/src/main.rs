use std::collections::HashMap;

fn main() {}

fn backtrace_fib(num: u128) -> u128 {
    if num == 0 || num == 1 {
        return num;
    }

    backtrace_fib(num - 1) + backtrace_fib(num - 2)
}

fn backtrace_memo_fib(num: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    match memo.get(&num) {
        Some(result) => *result,
        None => {
            let result = match num {
                0 | 1 => num,
                n => backtrace_memo_fib(n - 1, memo) + backtrace_memo_fib(n - 2, memo),
            };
            memo.insert(num, result.clone());
            result
        }
    }
}

fn dynamic_fib(num: u128) -> u128 {
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    match num {
        0 | 1 => {}
        n => {
            for i in 2..=n {
                let result = *memo.get(&(i - 1)).unwrap() + *memo.get(&(i - 2)).unwrap();
                memo.insert(i, result);
            }
        }
    };
    *memo.get(&num).unwrap()
}
