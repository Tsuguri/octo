use std::fmt;

pub fn helpo() {
    println!("heh");
}
#[derive(Debug)]
pub enum LexicalError {
    IsVeryBad,
    OpenComment(usize),
    UnexpectedCharacter(usize, char),
    OpenStringLiteral(usize),
    LiteralIntOverflow(usize, usize),
    LiteralFloatOverflow(usize, usize),
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match *self {
            LexicalError::IsVeryBad => "izverybad",
            LexicalError::OpenComment(_) => "Not closed block comment",
            LexicalError::UnexpectedCharacter(_, _) => "Unexpected character",
            LexicalError::OpenStringLiteral(_) => "Not closed string literal",
            LexicalError::LiteralIntOverflow(_, _) => "Literal int overflowed",
            LexicalError::LiteralFloatOverflow(_, _) => "Literal float overflow",
        };
        val.fmt(f)
    }
}

#[derive(Debug)]
pub struct Lines<'a> {
    offsets: Vec<usize>,
    end: usize,
    source: &'a str,
}

impl<'a> Lines<'a> {
    pub fn new(src: & str) -> Lines {
        let mut offsets = vec![];
        let src_len = src.len();
        offsets.push(0);
        let mut last = 0;
        for line in src.split('\n') {
            let len = line.len();
            offsets.push(len + last + 1);
            last = len + last + 1;
        }
        let _lines_count = offsets.len();
        // if lines_count > 1 {
        //     if offsets[lines_count - 1] == offsets[lines_count - 2] + 1 {
        //         offsets.pop();
        //     }
        // }
        //println!("{:?}", offsets);
        Lines {
            offsets,
            end: src_len,
            source: src,
        }
    }

    pub fn lines(&self) -> usize {
        self.offsets.len() - 1
    }

    pub fn get_line(&self, position: usize) -> Option<(usize, usize, usize)> {
        let line = self.offsets.iter().position(|x| x > &position);
        match line {
            None => Option::None,
            Some(i) => Option::Some((i - 1, self.offsets[i - 1], self.offsets[i])), // i will be always at least 1
        }
    }
    pub fn get_line_span(&self, line: usize) -> Option<(usize, usize)> {
        match line {
            x if x >= self.lines() => None, // invalid line (after EOF)
            x => Some((self.offsets[x], self.offsets[x + 1] - 1)),
        }
    }
}

fn show_code_snippet(
    line: usize,
    lines: &Lines,
    underscore: (usize, usize),
    help: Option<&str>,
) {
    //
    //println!("Entire code lenght: {}", src.len());
    let snippet = match line {
        0 => [0, 1, 2],                                   // first line
        n if n + 1 == lines.lines() => [n - 2, n - 1, n], // last line
        n => [n - 1, n, n + 1],                           // all others
    };
    for snip in snippet.iter() {
        let (from, to) = lines.get_line_span(*snip).unwrap();
        // println!(
        //     "start: {}, end: {}, from: {}, to: {}",
        //     underscore.0, underscore.1, from, to
        // );
        let content = &lines.source[from..to];
        print!("{:4}|", snip + 1);
        println!("{}", content);
        if from <= underscore.0 && to >= underscore.1 {
            let spaces = " ".repeat(5 + underscore.0 - from);
            print!("{}", spaces);
            let underscore = "^".repeat(underscore.1 - underscore.0 + 1);
            print!("{}", underscore);
            if let Some(message) = help {
                print!(" {}", message);
            }
            println!();
        }
    }
}

pub fn show_location(
    filepath: &str,
    lines: &Lines,
    message: &str,
    location: (usize, usize),
    help: Option<&str>,
) {
    println!("error: {}", message);
    match lines.get_line(location.0) {
        None => println!("in file: {} at unknown position", filepath),
        Some((line, _, _)) => {
            println!("--> at: {},{}", filepath, line);
            show_code_snippet(line, lines, (location.0, location.1), help);
        }
    }
}
