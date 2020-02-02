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
    for p in &philosophers {
        p.eat();
    }
}
