#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::{Error, Stdout}, slice::Windows};

use crossterm::{terminal as term, cursor, queue, execute};



pub enum Motion {
    Up(i32), 
    Down(i32), 
    Left(i32), 
    Right(i32),
    Endline, 
    BeginLine, 
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Cmd,
}

// STRUCT DEF \\
pub struct Buffer {
    pub lines   :   Vec<String>, 
    pub dim     :   (usize, usize), 
    pub center  :   u16, 
    pub relLines:   Vec<u16>, 

    pub mode    :   Mode,
    pub active  :   bool, 
    pub cursPos :   (usize, usize),

    pub useLineX:   bool, 
    pub updated :   Vec<usize>,
    pub upAll   :   bool, }
// CONTRUCTORS \\
impl Buffer {
    pub fn default() -> Result<Self, Error> {
        let lines = vec![ String::new(), ];
        let mut dim = term::size()?;
        if dim.1 % 2 == 0 { dim.1 -= 1; } else { dim.1 -= 2; }
        let center = (dim.1 as u16) >> 1; 
        let dim = (dim.0 as usize, dim.1 as usize);
        let mut relLines = Vec::new();

        for num in -(center as i32)..=(center as i32) {
            relLines.push(num.abs() as u16);
        }

        let mode = Mode::Normal;
        let active = true;
        let cursPos: (usize, usize) = (0, 0);
        let useLineX = false;

        let updated = Vec::new();
        let upAll = true;
        Ok(Self {
            lines, 
            dim, 
            center, 
            relLines, 
            mode, 
            active, 
            cursPos, 
            useLineX, 
            updated, 
            upAll, 
        })
    }}

// PUB FUNCS \\
impl Buffer {
    // MAIN RUN METHOD \\
    pub fn run<F>(self: &mut Self, mut f: F) -> Result<(), Error> where F: FnMut(&mut Self) -> Result<(), Error> {
        self.active = true;
        while self.active {
            f(self)?;
        }
        Ok(())}



    // move cursor \\ 
    pub fn moveCusor(self: &mut Self, motion: Motion) -> Result<(), Error> {
        let (mut x, mut y) = self.cursPos; 

        match motion {
            Motion::Up(c) => if y == 0 { return Ok(()); } else { y -= c as usize; self.upAll = true; }, 
            Motion::Down(c) => if y >= self.lines.len()-1 { return Ok(()); } else { y += c as usize; self.upAll = true; }, 
            Motion::Left(c) => if x == 0 || (self.useLineX && self.lines[y].is_empty()) { return Ok(()); } else {
                x -= c as usize;
                if self.useLineX {
                    x = self.lines[y].len()-1; 
                } }, 
            Motion::Right(c) => if x >= self.lines[y].len() { return Ok(()); } else { x += c as usize; }, 
            Motion::Endline => { x += 1000000; }, 
            Motion::BeginLine => { x = 0; },
        };

        let line = &self.lines[y];
        if x > line.len() { self.useLineX = true; } else { self.useLineX = false; }
        self.cursPos = (x, y);

        Ok(()) }

    pub fn setMode(self: &mut Self, out: &mut Stdout, mode: Mode) -> Result<(), Error> {
        match mode {
            Mode::Normal => {
                self.mode = Mode::Normal;
                queue!(out, cursor::SetCursorStyle::SteadyBlock)?;
            }, 

            Mode::Insert => {
                self.mode = Mode::Insert;
                queue!(out, cursor::SetCursorStyle::BlinkingBlock)?;
            }, 

            _=>(), }

        Ok(())}



    // ADDING \\
    pub fn insertLine(self: &mut Self, row: usize, text: &str) -> Result<(), Error>{
        self.lines.insert(row, String::from(text)); 
        self.updateToEnd(row);
        Ok(())}
    pub fn insertChar(self: &mut Self, c: char) {
        let (x, y) = self.cursPos;
        self.addUpdate(y);
        let line = &mut self.lines[y];

        if x >= line.len() {
            line.push(c);
            self.useLineX = false;
            self.cursPos = (line.len(), y);
            return; }

        line.insert(x, c); }
    pub fn insertText(self: &mut Self, text: &str) {
        let (x, y) = self.cursPos;
        self.addUpdate(y);
        let line = &mut self.lines[y];
        
        if x >= line.len() {
            line.push_str(text);
            self.useLineX = false;
            self.cursPos = (line.len(), y);
            return; }

        line.insert_str(x, text); }



    // DELETING \\
    pub fn deleteChar(self: &mut Self) -> Result<bool, Error> {
        let (x, y) = self.cursPos;
        if self.lines[y].len() == 0 { return Ok(false); }

        self.addUpdate(y);
        self.lines[y].remove(x);
        if x >= self.lines[y].len() { self.moveCusor(Motion::Left(1))?; }

        Ok(true) }
    pub fn deleteLine(self: &mut Self) -> Result<(), Error> {
        let (_, y) = self.cursPos;
        if self.lines.len() <= 1 { return Ok(()); }

        self.updateToEnd(y);
        self.lines.remove(y);
        if y >= self.lines.len() { self.moveCusor(Motion::Up(1))?; }
        Ok(()) }



    pub fn addUpdate(self: &mut Self, index: usize) { self.updated.push(index); }
    pub fn end(self: &mut Self) { self.active = false; }
    pub fn drawCursor(self: &Self, out: &mut Stdout) -> Result<(), Error> {
        let (mut x, y) = self.cursPos;
        if self.useLineX { x = self.lines[y].len(); }

        execute!(out, cursor::MoveTo(x as u16+5, self.center))?;

        Ok(()) }
    pub fn updateToEnd(self: &mut Self, from: usize) {
        for i in from..self.lines.len() {
            self.addUpdate(i);
        } }
}



