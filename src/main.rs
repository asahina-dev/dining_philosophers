use std::thread;
use std::time::Duration;

// 構造体
struct Philosopher {
    name: String,
}

// 指定した構造体に対する定義
impl Philosopher {
    // Philosopherインスタンスを返す
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    fn eat(&self) {
        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    // Vec<T>: 可変長の配列型
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    // _ は型プレースホルダ
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        })
        .collect(); // コレクション型を生成

    for h in handles {
        h.join().unwrap();
    }
}
