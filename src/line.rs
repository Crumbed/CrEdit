#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fmt::Display;

use termion::color;



pub enum Token {
    Tab { start: usize, count: u16 }, 
    Word { start: usize, len: usize, text: String }, 
    Other(usize, char), 
    Space(usize, u16), 
    None, 
}
impl Token {
    pub fn getLen(&self) -> usize {
        match self {
            Token::Tab { count, .. } => *count as usize,
            Token::Word { len, .. } => *len,
            Token::Other(..) => 1,
            Token::Space(_, count) => *count as usize, 
            Token::None => 1, 
        } }
}



pub struct Line {
    pub indent  :   u16, 
    pub tokens  :   Vec<Token>, 
    pub rawLine :   String, 
    pub dispLine:   String, 
    pub currTkn :   Token, }

// CONSTRUCTORS & PRIVATES \\
impl Line { 
    pub fn new(indent: u16) -> Self {
        let mut tokens = Vec::new();
        if indent > 0 { tokens.push(Token::Tab { start: 0, count: indent }); }
        let mut rawLine = String::new();
        let mut dispLine = String::new();
        for _ in 0..indent {
            rawLine.push_str("    ");
            dispLine.push_str("\t");
        }
        let currTkn = Token::None;
        tokens.push(currTkn);

        Self {
            indent, 
            tokens, 
            rawLine, 
            dispLine,
            currTkn, 
        } }

    pub fn from(indent: u16, text: &str) -> Self {
        let mut tokens = Line::tokenize(text);
        if indent > 0 { tokens.push(Token::Tab { start: 0, count: indent }); }

        let mut rawLine = String::from(text);
        let mut dispLine = Line::formatText(&rawLine);
        let mut currTkn = Token::None;
        if let Token::Word{ start, len, text } = tokens.last().unwrap() {
            currTkn = Token::Word { start: *start, len: *len, text: text.to_string() } }
        else { tokens.push(currTkn); }

        Self {
            indent, 
            tokens, 
            rawLine, 
            dispLine,
            currTkn, 
        } }
}

// PUB METHODS \\
impl Line {
    pub fn len(self: &mut Self) -> usize { self.rawLine.len() }

    pub fn add(&mut self, c: char) {
        self.rawLine.push(c);
        self.dispLine.push(c);
        if Line::isOther(c) {
            self.tokens.push(self.currTkn);
            self.tokens.push(Token::Other(self.len(), c));
            self.currTkn = Token::None;
            return; }
        match self.currTkn {
            Token::Word { start, len, text } => {
                let mut newTxt = text;
                newTxt.push(c);
                self.currTkn = Token::Word {
                    start, 
                    len, 
                    text: newTxt
                } }, 
            _=> { self.currTkn = Token::Word { start: self.len()-1, len: 1, text: String::from(c) } 
            } }

        self.tokens.pop();
        self.tokens.push(self.currTkn) }


    pub fn ins(&mut self, i: usize, c: char) {
        self.rawLine.insert(i, c);
        self.dispLine.insert(i, c);

        let tknIndex = self.tokens.len(); 
        let wordOffset: usize = 0;
        let offset: usize = 0;
        for (idx, tkn) in self.tokens.iter().enumerate() {
            offset += tkn.getLen();
            if offset >= i { wordOffset = i - offset - tkn.getLen(); }
            // TODO HERE
        }
        //     match tkn {
        //         Token::Word { start, len, .. } => if start <= &i {
        //             tknIndex = idx;
        //             offset = i - start;
        //             break; 
        //         },

        //         Token::Tab { start, count } => if &i <= &(start + *count as usize*4) && start <= &i {
        //             tknIndex = idx; 
        //             break;
        //         },

        //         _=>(), } }
        let tkn = &mut self.tokens[tknIndex];
        
        if Line::isOther(c) {
            // self.tokens.push(self.currTkn);
            // self.tokens.push(Token::Other(self.len(), c));
            // self.currTkn = Token::None;
            return; }

        
    }
}



// PUB FUNCS \\
impl Line {
    pub fn tokenize(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = text.chars();
        let mut curr = chars.next();
        let mut index: usize = 0;
        let mut currWord = String::new();

        let mut check = true;
        while check {
            let mut c = match curr {
                Some(ch) => ch,
                None => break, };

            while !Line::isOther(c) {
                currWord.push(c);
                curr = chars.next();
                index += 1;
                if let Some(ch) = curr { c = ch; }
                else {
                    tokens.push(Token::Word { start: index-1-currWord.len(), len: currWord.len(), text: currWord });
                    currWord.clear();
                    check = false;
                }
            }
            if !check { break; } 

            let word = Token::Word { start: index-1-currWord.len(), len: currWord.len(), text: currWord };
            if !currWord.is_empty() { tokens.push(word); }
            if c != ' ' { tokens.push(Token::Other(index, c)); }
            currWord.clear();

            let mut spcCount = 0;
            while c == ' ' {
                spcCount += 1;
                curr = chars.next();
                index += 1;
                if let Some(ch) = curr { c = ch; }
                else { check = false; } 
            }

            let ind: u16 = spcCount / 4;
            let extra: u16 = spcCount % 4;
            if ind > 0 { tokens.push(Token::Tab{ start:index-1-ind as usize, count:ind }); }
            for i in 0..extra { tokens.push(Token::Space(index-1, extra)); }
            if !check { break; } 

        }


        tokens
    }
    pub fn isOther(c: char) -> bool {
        match c {
            ' ' => true,
            '>' => true,
            '<' => true,
            '!' => true,
            ';' => true, 
            '(' => true,
            ')' => true,
            '[' => true,
            ']' => true,
            '{' => true,
            '}' => true,
            '=' => true,
            '+' => true,
            '-' => true,
            '*' => true,
            '/' => true,  

            _ => false
        } }



    pub fn formatText(text: &str) -> String {
        let mut fStr = String::from(text);

        while fStr.contains("    ") {
            fStr = fStr.replace("    ", &format!("{}····{}", color::Fg(color::LightBlack), color::Fg(color::White)));
        }

        fStr }
}



impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dispLine)
    }
}


























