use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use tokio::runtime::Runtime;

async fn log(char: char, target: &str, ans: Arc<Mutex<String>>, pointer: Arc<Mutex<usize>>) {
    let mut ans = ans.lock().unwrap();
    let mut pointer = pointer.lock().unwrap();
    
    let cur = target.chars().nth(*pointer).unwrap_or(' ');

    if cur == ' ' {
        *pointer += 1;
        ans.push(' ');
    }

    if char == cur {
        ans.push(char);
        *pointer += 1;
    }

    let to_log = if ans.ends_with("dd") {
        ans[..ans.len() - 1].to_string() + &char.to_string()
    } else {
        ans.clone() + &char.to_string()
    };

    println!("{}", to_log);

    if *ans == target {
        println!("Successfully logged 'hello world!'");
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let target = "hello world";
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let ans = Arc::new(Mutex::new(String::new()));
    let pointer = Arc::new(Mutex::new(0));
    
    let mut index = 0;

    while *pointer < target.len() {
        let char = chars[index];
        let ans = Arc::clone(&ans);
        let pointer = Arc::clone(&pointer);

        log(char, target, ans, pointer).await;

        index = (index + 1) % chars.len();
        sleep(Duration::from_millis(10)).await;
    }
}
