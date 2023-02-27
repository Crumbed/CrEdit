#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{time::Instant, collections::HashMap, ops, io::{Error, Stdout}};

use crate::buffer::{Mode, Buffer, Motion};




#[derive(Clone)]
#[derive(PartialEq)]
pub enum QuickAction {
    QuickNormal, 
    MakeBlock, 

    NAQA,
}
#[derive(Clone)]
pub enum Operation {
    ToInsert, 
    Insert(String),
    ToNormal, 
    EnterCmd, 

    Delete, 
    NewLine(Motion), 

    Up,
    Down,
    Left,
    Right,
}



#[derive(Clone)]
pub struct OperationBuffer {
    pub imacros :   HashMap<String, QuickAction>, 
    pub macStart:   Vec<char>,
    // ops     :   HashMap<String, Operation>, 
    pub currMac :   Vec<char>, 
    pub currOp  :   Vec<Operation>, 
    pub lastOp  :   Vec<Operation>, 
    pub lastInp :   Instant, }

// CONSTRUCTORS & PRIV FUNCS \\
impl OperationBuffer {
    pub fn new() -> Self {
        let mut imacros = HashMap::from([
            (String::from("jk"), QuickAction::QuickNormal), 
            (String::from("{\n"), QuickAction::MakeBlock),
        ]);
        let mut macStart = Vec::new();

        for imac in imacros.keys() {
            let start = imac.chars().next().unwrap(); 
            macStart.push(start);
        }

        // let mut ops = HashMap::from([
        // ]);

        Self {
            imacros, 
            macStart,
            // ops, 
            currMac :   Vec::new(), 
            currOp  :   Vec::new(), 
            lastOp  :   Vec::new(), 
            lastInp :   Instant::now(), 
        } }
}

// PUB FUNCS \\ 
impl OperationBuffer {
    // pub fn checkOperation(self: &mut Self, operation: Operation) {
        
    // }

    pub fn startMacro(self: &mut Self, c: char) -> Option<QuickAction> {
        self.currMac.push(c);
        self.lastInp = Instant::now();
        self.checkMacro(c)
    }

    pub fn checkMacro(self: &mut Self, c: char) -> Option<QuickAction> {
        let mut posMacs = self.imacros.clone();
        if self.lastInp.elapsed().as_millis() >= 250 { return Some(QuickAction::NAQA); }
        for i in 0..self.currMac.len() {
            for imac in posMacs.clone().keys() {
                let key = imac.chars().nth(i);
                if let Some(chtr) = key {
                    if chtr != self.currMac[i] { posMacs.remove(imac); }
                } else { break; }
            }
        }
        if posMacs.is_empty() { return Some(QuickAction::NAQA); }

        for imac in posMacs.keys() {
            let key = imac.chars().nth(self.currMac.len());
            if let Some(chtr) = key {
                if chtr != c { return None; }
                self.currMac.push(c);
                self.lastInp = Instant::now();
                break;
            } 
        }

        let currMacStr: String = self.currMac.iter().collect();
        return match self.imacros.get(&currMacStr) {
            Some(action) => Some(action.clone()),
            None => None
        }
    }
    
    pub fn resetMacro(self: &mut Self) {
        self.currMac.clear();
    }




    
    pub fn executeMaco(self: &mut Self, buf: &mut Buffer, action: QuickAction) -> Result<(), Error> {
        match action {
            QuickAction::QuickNormal => {
                buf.opStream.push(Operation::ToNormal);
                self.resetMacro();
            },
            QuickAction::MakeBlock => {
                buf.opStream.push(Operation::Insert("{}".to_string())); // insert mode and add text (auto move cursor)
                buf.opStream.push(Operation::Left); // move cursor left
                buf.opStream.push(Operation::NewLine(Motion::Down(1))); // newline down (carry } to newline because insert mode)
                buf.opStream.push(Operation::ToNormal); // to normal mode
                buf.opStream.push(Operation::NewLine(Motion::Up(1))); // newline up (dont carry } because normal mode)
                buf.opStream.push(Operation::Insert("    ".to_string())); // insert mode and add text (auto move cursor)
                self.resetMacro();
            },

            _ => todo!(),
        }

        Ok(())
    }

}
