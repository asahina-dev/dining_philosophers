use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 構造体
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

// 指定した構造体に対する定義
impl Philosopher {
    // Philosopherインスタンスを返す
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    fn eat(&self, table: &Table) {
        // 値を使う予定がない場合は頭に _ をつける(警告されなくなる)
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    // Arc: アトミック参照
    // 複数スレッドから対象を共有するために必要となる。共有するときは参照カウントを増やし、 各スレッドの終了時にはカウントを減らす。
    let table = Arc::new(Table {
        // Vec<T>: 可変長の配列型
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        // デッドロックを防ぐため rightを0 にした
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // _ は型プレースホルダ
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect(); // コレクション型を生成

    for h in handles {
        h.join().unwrap();
    }
}
