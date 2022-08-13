#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use rand::{thread_rng, Rng};
use std::fmt;
use std::io::stdin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

enum State {
    Locked,
    Failed,
    Unlocked,
}

fn lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }

                    Err(_) => continue,
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
}

fn number_guessing_game() {
    let mut rng = thread_rng();
    let n: i64 = rng.gen_range(1..101);

    loop {
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parse = buffer.trim_end().parse::<i64>();
                match parse {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Out of range");
                        } else if guess < n {
                            println!("Guess is too low");
                        } else if guess > n {
                            println!("Guess is too high");
                        } else {
                            println!("Correct! The number is {}", guess);
                            break;
                        }
                    }

                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            Err(_) => {
                continue;
            }
        }
    }
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn closures() {
    let plus_one = |x: i32| -> i32 { x + 1 };

    let mut two = 2;

    let plus_two = |x: i32| {
        let mut z = x;
        z += two;
        z
    };

    let borrow_two = &mut two;
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn lambda_sum() {
    let limit = 500;
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
}

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

struct Actor {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Actor {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Actor {
        Actor {
            name: name,
            state: state,
        }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

fn main() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let actor = Actor::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        actor.greet();
    });

    println!(
        "Name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );

    t.join().unwrap();
}
