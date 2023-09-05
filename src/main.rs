use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} •••", self.name);

        thread::sleep(Duration::from_secs(1));

        println!("{} ✅", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    loop {
        let philosophers = vec![
            Philosopher::new("Kant", 0, 1),
            Philosopher::new("Nietzsche", 1, 2),
            Philosopher::new("Marx", 2, 3),
            Philosopher::new("Socrates", 3, 4),
            Philosopher::new("Plato", 0, 4),
        ];

        let handles: Vec<_> = philosophers
            .into_iter()
            .map(|p| {
                let table = table.clone();
        
                thread::spawn(move || {
                    p.eat(&table);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
