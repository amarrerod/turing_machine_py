use itertools::Itertools;
use pyo3::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct State {
    pub id: u32,
}

#[pymethods]
impl State {
    #[new]
    pub fn new(id: u32) -> Self {
        State { id }
    }

    fn __repr__(&self) -> String {
        format!("State ({})", self.id)
    }

    fn __str__(&self) -> String {
        self.id.to_string()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub enum Moves {
    L,
    R,
    S,
}

impl std::fmt::Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Moves::L => write!(f, "L"),
            Moves::R => write!(f, "R"),
            Moves::S => write!(f, "S"),
        }
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub state: State,
    pub read_symb: char,
    pub write_symb: char,
    pub _move: Moves,
    pub next_state: State,
}

#[pymethods]
impl Tuple {
    #[new]
    pub fn new(
        state: State,
        read_symb: char,
        write_symb: char,
        _move: Moves,
        next_state: State,
    ) -> Self {
        Tuple {
            state,
            read_symb,
            write_symb,
            _move,
            next_state,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Tuple ({}, {}, {}, {}, {})",
            self.state, self.read_symb, self.write_symb, self._move, self.next_state
        )
    }

    fn __str__(&self) -> String {
        format!(
            "Tuple ({}, {}, {}, {}, {})",
            self.state, self.read_symb, self.write_symb, self._move, self.next_state
        )
        .to_string()
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Tape {
    pub content: Vec<char>,
    pub pos: u32,
    pub white_char: char,
}

#[pymethods]
impl Tape {
    #[new]
    pub fn new(content: Vec<char>, white_char: char) -> Tape {
        Tape {
            content,
            pos: 0,
            white_char,
        }
    }

    fn __repr__(&self) -> String {
        format!("Tape ({:?})", self.content)
    }

    fn __str__(&self) -> String {
        self.content.iter().collect()
    }

    pub fn move_head(&mut self, dir: Moves) {
        match dir {
            Moves::L => self.move_left(),
            Moves::R => self.move_right(),
            Moves::S => {}
        }
    }

    fn move_right(&mut self) {
        if self.pos as usize == self.content.len() - 1 {
            self.content.push(self.white_char);
        }
        self.pos += 1;
    }

    fn move_left(&mut self) {
        if self.pos == 0 {
            let mut new_content: Vec<char> = vec![self.white_char; 1];
            new_content.append(&mut self.content);
            self.content = new_content;
        }
        self.pos -= 1;
    }

    pub fn set_char_at_pos(&mut self, new_char: char) {
        self.content[self.pos as usize] = new_char;
    }

    pub fn get_char_at_pos(&self) -> char {
        self.content[self.pos as usize]
    }
}

impl std::fmt::Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

#[pyclass]
pub struct TuringMachine {
    states: Vec<State>,
    inital_state: State,
    final_states: Vec<State>,
    input_alph: Vec<char>,
    tuples: Vec<Tuple>,
    empty_space: char,
    tape: Tape,
    current_state: State,
}

#[pymethods]
impl TuringMachine {
    #[new]
    pub fn new(
        states: Vec<State>,
        inital_state: State,
        final_states: Vec<State>,
        input_alph: Vec<char>,
        tuples: Vec<Tuple>,
        empty_space: char,
        tape: Tape,
    ) -> TuringMachine {
        let current_state: State = State::new(inital_state.id);
        TuringMachine {
            states,
            inital_state,
            final_states,
            input_alph,
            tuples,
            empty_space,
            tape,
            current_state,
        }
    }

    pub fn run(&mut self) -> PyResult<Tape> {
        loop {
            let current_char: char = self.tape.get_char_at_pos();
            let tuple: Option<&Tuple> = self
                .tuples
                .iter()
                .filter(|t| (t.read_symb == current_char) && (t.state == self.current_state))
                .next();
            if tuple == None {
                break;
            }
            let tuple: Tuple = tuple.unwrap().clone();
            self.tape.set_char_at_pos(tuple.write_symb);
            self.tape.move_head(tuple._move);
            self.current_state = tuple.next_state;
        }
        if !self.final_states.contains(&self.current_state) {
            Error::new(ErrorKind::Other, "Finished in a non-final state");
        }
        Ok(self.tape.clone())
    }

    fn __repr__(&self) -> String {
        format!(
            "Turing Machine\n - States: {:?} \n - Initial State: {} \n Final States: {:?} \n Alphabet: {:?}  \n Tuples: {:#?} \n White char: {} \n Current Tape: {} \n Current State: {} \n",
            self.states, self.inital_state, self.final_states, self.input_alph, self.tuples, self.empty_space, self.tape, self.current_state
        )
    }

    fn __str__(&self) -> String {
        format!(
            "Turing Machine\n - States: {:?} \n - Initial State: {} \n Final States: {:?} \n Alphabet: {:?}  \n Tuples: {:#?} \n White char: {} \n Current Tape: {} \n Current State: {} \n",
            self.states, self.inital_state, self.final_states, self.input_alph, self.tuples, self.empty_space, self.tape, self.current_state
        )
    }
}

#[pyfunction]
pub fn create_states(n_states: u32) -> Vec<State> {
    (0..n_states).map(|i| State::new(i as u32)).collect()
}

#[pyfunction]
pub fn create_final_states(ids: Vec<&str>) -> Vec<State> {
    return ids
        .into_iter()
        .map(|i| State::new(i.parse::<u32>().unwrap()))
        .collect();
}

#[pyfunction]
pub fn create_tuple(definition: &str) -> Tuple {
    let tokens: Vec<&str> = definition.split_whitespace().collect();
    Tuple {
        state: State::new(tokens[0].parse::<u32>().unwrap()),
        read_symb: tokens[1].chars().next().expect("Read symbol is empty"),
        write_symb: tokens[2].chars().next().expect("Write symbol is empty"),
        _move: match tokens[3] {
            "S" => Moves::S,
            "R" => Moves::R,
            "L" => Moves::L,
            _ => panic!("Move not recognized!"),
        },
        next_state: State::new(tokens[4].parse::<u32>().unwrap()),
    }
}

#[pyfunction]
pub fn load_tape_from_file(filename: &str) -> PyResult<Tape> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    let data: String = file_reader.lines().collect::<Result<_, _>>().unwrap();
    let default_white: char = '$';
    let tape: Tape = Tape::new(data.chars().collect(), default_white);
    Ok(tape)
}

#[pyfunction]
pub fn load_from_instance(
    tm_filename: &str,
    tape_filename: &str,
) -> Result<TuringMachine, io::Error> {
    let file = File::open(tm_filename)?;
    let file_reader = BufReader::new(file);
    let data: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();

    let mut states: Vec<State> = vec![];
    let mut initial_state: State = State::new(0);
    let mut final_states: Vec<State> = vec![];
    let mut tuples: Vec<Tuple> = vec![];
    let mut white_space: char = '$';
    let mut alpha: Vec<char> = vec![];

    for (index, line) in data.iter().enumerate() {
        match index {
            0 => states = create_states(line.parse::<u32>().unwrap()),
            1 => initial_state = State::new(line.parse::<u32>().unwrap()),
            2 => final_states = create_final_states(line.split(',').collect()),
            3 => white_space = line.chars().next().expect("No white space given"),
            _ => {
                let tuple: Tuple = create_tuple(line);
                alpha.push(tuple.read_symb);
                alpha.push(tuple.write_symb);
                tuples.push(tuple);
            }
        }
    }
    let alpha: Vec<char> = alpha.into_iter().unique().collect();
    let mut tape: Tape = load_tape_from_file(tape_filename).unwrap();
    tape.white_char = white_space;
    let tm: TuringMachine = TuringMachine::new(
        states,
        initial_state,
        final_states,
        alpha,
        tuples,
        white_space,
        tape,
    );
    Ok(tm)
}
