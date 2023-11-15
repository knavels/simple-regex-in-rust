mod fsm {
    pub use std::ops::Range;

    pub const FSM_COLUMN_SIZE: usize = 130;
    pub const FSM_NEWLINE: usize = 129;

    pub type Index = usize;

    pub struct Column {
        pub ts: [Index; FSM_COLUMN_SIZE],
    }

    impl Column {
        pub fn new() -> Self {
            Self {
                ts: [0; FSM_COLUMN_SIZE],
            }
        }

        pub fn fill_range(&mut self, range: Range<char>, state: Index) {
            for i in range {
                self.ts[i as usize] = state;
            }
        }
    }

    pub struct Fsm {
        pub cs: Vec<Column>,
    }

    impl Fsm {
        pub fn new() -> Self {
            Self { cs: Vec::new() }
        }

        pub fn push(&mut self, column: Column) {
            self.cs.push(column);
        }

        pub fn dump(&mut self) {
            for symbol in 0..FSM_COLUMN_SIZE {
                print!("{:03} => ", symbol);
                for column in self.cs.iter() {
                    print!("{} ", column.ts[symbol]);
                }

                println!("");
            }
        }
    }
}

fn match_fsm(fsm: &fsm::Fsm, input: &str) -> bool {
    let mut state = 1;

    for c in input.chars() {
        if state == 0 || state >= fsm.cs.len() {
            break;
        }

        state = fsm.cs[state].ts[c as usize];
    }

    if state == 0 {
        return false;
    }

    if state < fsm.cs.len() {
        state = fsm.cs[state].ts[fsm::FSM_NEWLINE];
    }

    return state >= fsm.cs.len();
}

fn main() {
    let mut fsm = fsm::Fsm::new();

    let events = vec!['a' as usize, 'b' as usize, 'c' as usize, fsm::FSM_NEWLINE];

    // Failed state
    fsm.push(fsm::Column::new());

    for event in events.iter() {
        let mut col = fsm::Column::new();
        col.ts[*event] = fsm.cs.len() + 1;
        fsm.push(col);
    }

    fsm.dump();

    let inputs = vec!["Hello, World!", "abc", "abcd"];

    for input in inputs.iter() {
        println!("{:?} => {:?}", input, match_fsm(&fsm, input));
    }
}
