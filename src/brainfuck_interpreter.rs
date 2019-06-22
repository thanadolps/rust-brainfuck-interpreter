use std::fmt::{Debug, Error, Formatter};
use std::str::CharIndices;

pub struct BrainFuck<'a> {
    tape: Vec<i32>,
    pos: usize,
    callback_fn: Option<&'a Fn(&Self)>,
    last_instruct: Option<(usize, char)>,
    output_buffer: String,
}

impl<'a> BrainFuck<'a> {
    pub fn new() -> Self {
        BrainFuck {
            tape: vec![0],
            pos: 0,
            callback_fn: None,
            last_instruct: None,
            output_buffer: String::new(),
        }
    }

    pub fn callback(&mut self, call: &'a Fn(&Self)) -> &mut Self {
        self.callback_fn = Some(call);
        self
    }

    fn run_callback(&self) {
        if let Some(i) = self.callback_fn {
            i(self)
        }
    }

    fn current_val(&self) -> i32 {
        self.tape[self.pos]
    }

    fn current_val_mut(&mut self) -> &mut i32 {
        self.tape.get_mut(self.pos).expect("index out of bound")
    }

    fn move_right(&mut self) {
        if self.pos + 1 >= self.tape.len() {
            self.tape.push(0)
        }
        self.pos += 1
    }

    fn move_left(&mut self) {
        self.pos -= 1
    }

    pub fn interpret(&mut self, code: &'a str) -> &str {
        self._interpret(code, false);
        self.output_buffer.as_str()
    }

    // get a slice of code in bracket with [ at start_pos and chars a iterable starting at start_pos
    // code: ++[--+++], start_pos: 2, chars: CharIndices at 2  -> --+++
    fn slice_bracket(code: &'a str, start_pos: usize, chars: &mut CharIndices) -> &'a str {

        // find position of matching ]
        let mut bracket_depth = 0u8;
        let end_pos = loop {
            let (i, c) = chars.next().unwrap();
            match c {
                '[' => bracket_depth += 1,
                ']' => if bracket_depth <= 0 { break i; } else { bracket_depth -= 1 }
                _ => (),
            }
        };

        // slice it
        &code[start_pos + 1..end_pos]
    }

    fn _interpret(&mut self, code: &'a str, in_bracket: bool) {
        let mut chars = code.char_indices();
        // set to Some(i) when [ is founded where i is position of [
        // reset to None after matching ] is found (in bracket scanning)
        let mut bracket_scan = None;

        // callback thing
        self.run_callback();

        loop {
            // bracket scanning (found [ on last iteration)
            if let Some(start) = bracket_scan {
                bracket_scan = None;
                let sliced_code = Self::slice_bracket(code, start, &mut chars);
                self._interpret(sliced_code, true);
            }
            // normal processing
            else if let Some((i, x)) = chars.next() {
                let cls =
                    |this: &mut Self| {
                        this.last_instruct = Some((i, x));
                        this.run_callback()
                    };

                match x {
                    '+' => {
                        *self.current_val_mut() += 1;
                        cls(self)
                    }
                    '-' => {
                        *self.current_val_mut() -= 1;
                        cls(self)
                    }
                    '>' => {
                        self.move_right();
                        cls(self)
                    }
                    '<' => {
                        self.move_left();
                        cls(self)
                    }
                    '.' => self.output_buffer.push(self.current_val() as u8 as char),
                    '?' => self.output_buffer.push_str(format!("{}", self.current_val()).as_str()),
                    '[' => bracket_scan = Some(i),
                    _ => (),
                }
            }
            // EOF but in [] and need to loop again
            else if in_bracket && self.current_val() != 0 {
                // reset chars
                chars = code.char_indices();
            }
            // True end
            else {
                break;
            }
        }
    }
}

impl<'a> Debug for BrainFuck<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if let Some((_, j)) = self.last_instruct {
            write!(f, "pos:{}, last: {}\n{:?}", self.pos, j, self.tape)
        } else {
            write!(f, "pos:{}\n{:?}", self.pos, self.tape)
        }
    }
}