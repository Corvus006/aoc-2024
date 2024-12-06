use crate::day_03::read_file;

pub fn task2(input: String) -> i32 {
    let code = match read_file(input) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return 0;
        }
    };

    let mut out= 0;
    let mut parser = Parser::new(&code);
    let mut active = true;

    while !parser.is_eof(){
        active |= parser.expect("do()");
        active &= !parser.expect("don't()");

        if parser.expect("mul("){
            let Some(a) = parser.number() else {continue};
            if !parser.expect(","){
                continue;
            }
            let Some(b) = parser.number() else {continue};
            if !parser.expect(")"){
                continue;
            }

            if active {
                out += a*b;
            }
        }else {
            parser.advance(1);
        }
    }
    out
}

struct Parser {
    chars: Vec<char>,
    idx: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
        }
    }

    pub fn expect(&mut self, str: &str) -> bool {
        let valid = self.idx + str.len() < self.chars.len()
            && self.chars[self.idx..self.idx + str.len()]
            .iter()
            .zip(str.chars())
            .all(|(&a, b)| a == b);

        if valid {
            self.idx += str.len();
        }

        valid
    }

    pub fn number(&mut self) -> Option<i32> {
        let mut working = String::new();
        while self.chars[self.idx].is_ascii_digit() && self.idx < self.chars.len() {
            working.push(self.chars[self.idx]);
            self.idx += 1;
        }
        working.parse::<i32>().ok()
    }
    pub fn advance(&mut self, count: usize) {
        self.idx += count;
    }
    pub fn is_eof(&self) -> bool {
        self.idx >= self.chars.len()
    }
}