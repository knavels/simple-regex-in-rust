// https://en.wikipedia.org/wiki/Finite-state_machine
// https://en.wikipedia.org/wiki/Turing_machine

// Todo:: use enum instead of the current techniques

use std::io::{self, BufRead, Write};

const LOCKED: usize = 0;
const UNLOCKED: usize = 1;
const STATES_COUNT: usize = 2;

const PUSH: usize = 0;
const COIN: usize = 1;
const EVENTS_COUNT: usize = 2;

const FSM: [[usize; EVENTS_COUNT]; STATES_COUNT] = [
    //      [PUSH, COIN]
    /* 0 */ [LOCKED, UNLOCKED], // LOCKED
    //      [PUSH, COIN]
    /* 1 */ [LOCKED, UNLOCKED], // UnLOCKED
];

fn next_state(state: usize, event: usize) -> usize {
    FSM[state][event]
}

fn state_to_str(state: usize) -> &'static str {
    match state {
        LOCKED => "Locked",
        UNLOCKED => "UnLocked",
        _ => unreachable!(),
    }
}

pub fn main() {
    let mut state = LOCKED;

    println!("State: {}", state_to_str(state));
    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, COIN),
            "push" => state = next_state(state, PUSH),
            ":q" => break,
            unknown => {
                eprintln!("ERROR: Unknown event '{}'", unknown);
            }
        }

        println!("State: {}", state_to_str(state));
        print!("> ");
        io::stdout().flush().unwrap();
    }

    println!("bye! 👋");
}
