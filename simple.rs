use std::thread;
use std::time::Duration;

fn main() {
    let target = "hello world";
    let mut index = 0;
    let mut pointer = 0;
    let mut ans = String::new();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    
    loop {
        let cur = chars[index];
        index = (index + 1) % 26;
        let target_index = target.chars().nth(pointer).unwrap_or(' ');

        if target_index == ' ' {
            pointer += 1;
            ans.push(' ');
        }

        if cur == target_index {
            ans.push(cur);
            pointer += 1;
        }

        let to_log = if ans.ends_with("dd") {
            ans[..ans.len() - 1].to_string() + &cur.to_string()
        } else {
            ans.clone() + &cur.to_string()
        };

        println!("{}", to_log);

        if ans == target {
            println!("Successfully logged 'Hello World!'");
            break;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
