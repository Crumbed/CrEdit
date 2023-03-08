#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fmt::Display;

use termion::color;



#[derive(Clone, Debug)]
pub enum Token {
    // Tab { start: usize, count: u16 }, 
    Word { start: usize, len: usize, text: String }, 
    Other(usize, char), 
    Space(usize, u16), 
    None, 
}
impl Token {
    pub fn getLen(&self) -> usize {
        match self {
            Token::Word { len, .. } => *len,
            Token::Other(..) => 1,
            Token::Space(_, count) => *count as usize, 
            Token::None => 1, 
        } }
}



#[derive(Debug)]
pub struct Line {
    // pub indent  :   u16, 
    pub tokens  :   Vec<Token>, 
    pub rawLine :   String, 
    pub dispLine:   String, 
    // pub currTkn :   Token, 
}


// CONSTRUCTORS & PRIVATES \\
impl Line {
    pub fn new() -> Self {
        let tokens = Vec::new();
        let rawLine = String::new();
        let dispLine = String::new();

        Self {
            tokens, 
            rawLine, 
            dispLine, 
        }
    }
    pub fn from(txt: &str) -> Self {
        let tokens = Line::tokenize(txt);
        let rawLine = String::from(txt);
        let dispLine = String::from(txt); 



        Self {
            tokens, 
            rawLine, 
            dispLine, 
        }
    }
}


// METHODS \\
impl Line {
    pub fn ins(&mut self, i: usize, c: char) {
        self.rawLine.insert(i, c);
        let mut tknIndex = 0;
        let mut offsetIndex = 0;

        while (self.tokens[tknIndex].getLen() + offsetIndex) < i {
            offsetIndex += self.tokens[tknIndex].getLen();
            tknIndex += 1;
        }

        let token = self.tokens[tknIndex].clone();
        
        if token.getLen() > 1 {
            match token {
                Token::Word { start, len, text } => {
                    let mut newTkn: Token;
                    if Line::isOther(c) {
                        // TODO -- manage case where word index is 0
                    }
                },
                Token::Space(start, count) => {

                },
                _=>{}, 
            }
        }
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
                    tokens.push(Token::Word { start: index-currWord.len(), len: currWord.len(), text: currWord.clone() });
                    currWord.clear();
                    check = false;
                }
            }
            if !check { break; } 


            let word = Token::Word { start: index-currWord.len(), len: currWord.len(), text: currWord.clone() };
            if !currWord.is_empty() { tokens.push(word); }
            currWord.clear();

            if c != ' ' {
                tokens.push(Token::Other(index, c)); 
                index += 1;
                curr = chars.next();
                if let Some(ch) = curr { c = ch; }
                else { check = false; }
            }
            if !check { break; } 

            let mut spcCount = 0;
            while c == ' ' {
                spcCount += 1;
                curr = chars.next();
                index += 1;
                if let Some(ch) = curr { c = ch; }
                else { check = false; } 
            }

            tokens.push(Token::Space(index - spcCount, spcCount as u16));
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



// impl Display for Line {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.dispLine)
//     }
// }




// CONSTRUCTORS & PRIVATES \\
// impl Line { 
//     pub fn new(indent: u16) -> Self {
//         let mut tokens = Vec::new();
//         if indent > 0 { tokens.push(Token::Tab { start: 0, count: indent }); }
//         let mut rawLine = String::new();
//         let mut dispLine = String::new();
//         for _ in 0..indent {
//             rawLine.push_str("    ");
//             dispLine.push_str("\t");
//         }
//         let currTkn = Token::None;
//         tokens.push(currTkn.clone());

//         Self {
//             indent, 
//             tokens, 
//             rawLine, 
//             dispLine,
//             currTkn, 
//         } }

//     pub fn from(indent: u16, text: &str) -> Self {
//         let mut tokens = Line::tokenize(text);
//         if indent > 0 { tokens.push(Token::Tab { start: 0, count: indent }); }

//         let mut rawLine = String::from(text);
//         let mut dispLine = Line::formatText(&rawLine);
//         let mut currTkn = Token::None;
//         if let Token::Word{ start, len, text } = tokens.last().unwrap() {
//             currTkn = Token::Word { start: *start, len: *len, text: text.to_string() } }
//         else { tokens.push(currTkn.clone()); }

//         Self {
//             indent, 
//             tokens, 
//             rawLine, 
//             dispLine,
//             currTkn, 
//         } }
// }

// // PUB METHODS \\
// impl Line {
        
    
// }



// // PUB FUNCS \\
// impl Line {
//     pub fn tokenize(text: &str) -> Vec<Token> {
//         let mut tokens = Vec::new();
//         let mut chars = text.chars();
//         let mut curr = chars.next();
//         let mut index: usize = 0;
//         let mut currWord = String::new();

//         let mut check = true;
//         while check {
//             let mut c = match curr {
//                 Some(ch) => ch,
//                 None => break, };

//             while !Line::isOther(c) {
//                 currWord.push(c);
//                 curr = chars.next();
//                 index += 1;
//                 if let Some(ch) = curr { c = ch; }
//                 else {
//                     tokens.push(Token::Word { start: index-1-currWord.len(), len: currWord.len(), text: currWord });
//                     currWord.clear();
//                     check = false;
//                 }
//             }
//             if !check { break; } 

//             let word = Token::Word { start: index-1-currWord.len(), len: currWord.len(), text: currWord };
//             if !currWord.is_empty() { tokens.push(word); }
//             if c != ' ' { tokens.push(Token::Other(index, c)); }
//             currWord.clear();

//             let mut spcCount = 0;
//             while c == ' ' {
//                 spcCount += 1;
//                 curr = chars.next();
//                 index += 1;
//                 if let Some(ch) = curr { c = ch; }
//                 else { check = false; } 
//             }

//             let ind: u16 = spcCount / 4;
//             let extra: u16 = spcCount % 4;
//             if ind > 0 { tokens.push(Token::Tab{ start:index-1-ind as usize, count:ind }); }
//             for i in 0..extra { tokens.push(Token::Space(index-1, extra)); }
//             if !check { break; } 

//         }


//         tokens
//     }
//     pub fn isOther(c: char) -> bool {
//         match c {
//             ' ' => true,
//             '>' => true,
//             '<' => true,
//             '!' => true,
//             ';' => true, 
//             '(' => true,
//             ')' => true,
//             '[' => true,
//             ']' => true,
//             '{' => true,
//             '}' => true,
//             '=' => true,
//             '+' => true,
//             '-' => true,
//             '*' => true,
//             '/' => true,  

//             _ => false
//         } }



//     pub fn formatText(text: &str) -> String {
//         let mut fStr = String::from(text);

//         while fStr.contains("    ") {
//             fStr = fStr.replace("    ", &format!("{}····{}", color::Fg(color::LightBlack), color::Fg(color::White)));
//         }

//         fStr }
// }



// impl Display for Line {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.dispLine)
//     }
// }


























