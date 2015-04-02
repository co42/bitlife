#![feature(thread_sleep)]
#![feature(std_misc)]

use std::thread::sleep;
use std::time::duration::Duration;

use Address::{ Immediate, Direct, Indirect, PreIncIndirect, PostIncIndirect };
use Instruction::{ DAT, NOP, MOV, ADD, SUB, MUL, DIV, MOD, JMP, JMZ, JMN, DJN, CMP, SLT, SPL };
use Modifier::{ A, B, AB, BA, F, X, I };

// Address modes
#[derive(PartialEq, Clone)]
enum Address {
    Immediate,       // #
    Direct,          // $
    Indirect,        // @
    PreIncIndirect,  // { < TODO not implemented
    PostIncIndirect, // } > TODO not implemented
}

// Instructions
#[derive(PartialEq, Clone)]
enum Instruction {
    DAT,
    NOP,
    MOV,
    ADD, // TODO not implemented
    SUB, // TODO not implemented
    MUL, // TODO not implemented
    DIV, // TODO not implemented
    MOD, // TODO not implemented
    JMP,
    JMZ,
    JMN,
    DJN, // TODO not implemented
    CMP,
    SLT, // TODO not implemented
    SPL, // TODO not implemented
}

// Modifiers
#[derive(PartialEq, Clone)]
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

#[derive(PartialEq, Clone)]
struct Param {
    addr: Address,
    val:  i32,
}

impl Param {
    fn new(addr: Address, val: i32) -> Param {
        Param { addr: addr, val: val }
    }
}

#[derive(PartialEq, Clone)]
struct Cell {
    ins:      Instruction,
    modifier: Modifier,
    a:        Param,
    b:        Param,
}

impl Cell {
    fn new(ins: Instruction, opt_modifier: Option<Modifier>, a: Param, b: Param) -> Cell {
        let modifier = match opt_modifier {
            Some(modifier) => modifier,
            // ICWS'88 to ICWS'94 conversion
            None           => {
                match ins {
                    DAT | NOP                   => F,
                    MOV | CMP                   => {
                        match (&a.addr, &b.addr) {
                            (&Immediate, _) => AB,
                            (_, &Immediate) => B,
                            _               => I,
                        }
                    },
                    ADD | SUB | MUL | DIV | MOD => {
                        match (&a.addr, &b.addr) {
                            (&Immediate, _) => AB,
                            (_, &Immediate) => B,
                            _               => F,
                        }
                    },
                    SLT                         => {
                        if a.addr == Immediate {
                            AB
                        } else {
                            B
                        }
                    },
                    JMP | JMZ | JMN | DJN | SPL => B,
                }
            }
        };
        Cell { ins: ins, modifier: modifier, a: a, b: b }
    }
}

struct Mars {
    core: Vec<Cell>,
    iptr: usize,
}

impl Mars {
    fn new(size: usize) -> Mars {
        let default = Cell::new(DAT, None, Param::new(Immediate, 0), Param::new(Immediate, 0));
        Mars { core: vec![default; size], iptr: 0 }
    }

    fn load(&mut self, warrior: Vec<Cell>) {
        for (idx, cell) in warrior.iter().enumerate() {
            self.core[idx] = cell.clone();
        }
    }

    fn run(&mut self) {
        loop {
            let iptr = self.iptr;

            // Debug
            for (cptr, cell) in self.core.iter().enumerate() {
                if cptr == self.iptr {
                    print!("\x1B[0;31m");
                }
                match cell.ins {
                    DAT => print!("{} ", cell.b.val),
                    MOV => print!("M "),
                    ADD => print!("A "),
                    JMP => print!("J "),
                    _   => print!("U "),
                };
                if cptr == self.iptr {
                    print!("\x1B[0;0m");
                }
            }
            println!("");

            self.iptr = match self.execute(iptr) {
                Some(iptr) => iptr,
                None       => break,
            };
            sleep(Duration::milliseconds(100));
        }
    }

    fn param<F>(&self, iptr: usize, get_param: F) -> usize
        where F: Fn(&Cell) -> &Param {

        let ref cell = self.core[iptr];
        let param = get_param(cell);
        let ptr = match param.addr {
            Immediate       => 0,
            Direct          => param.val,
            Indirect        => {
                let ptrcell = self.add_iptr(iptr, param.val);
                param.val + get_param(&self.core[ptrcell]).val
            },
            PreIncIndirect  => panic!("PreIncIndirect not implemented"),
            PostIncIndirect => panic!("PostIncIndirect not implemented"),
        };
        self.add_iptr(iptr, ptr)
    }

    fn execute(&mut self, iptr: usize) -> Option<usize> {
        let cell = self.core[iptr].clone();

        // Parameters
        let aptr = self.param(iptr, |c| &c.a);
        let bptr = self.param(iptr, |c| &c.b);

        // Instruction
        match cell.ins {
            DAT => return None,
            NOP => { },
            MOV => match cell.modifier {
                A  => self.core[bptr].a.val = self.core[aptr].a.val,
                B  => self.core[bptr].b.val = self.core[aptr].b.val,
                AB => self.core[bptr].b.val = self.core[aptr].a.val,
                BA => self.core[bptr].a.val = self.core[aptr].b.val,
                F  => {
                    self.core[bptr].a.val = self.core[aptr].a.val;
                    self.core[bptr].b.val = self.core[aptr].b.val;
                },
                X  => {
                    self.core[bptr].b.val = self.core[aptr].a.val;
                    self.core[bptr].a.val = self.core[aptr].b.val;
                },
                I  => self.core[bptr] = self.core[aptr].clone(),
            },
            ADD => match cell.modifier {
                AB => {
                    let sum = self.core[aptr].a.val + self.core[bptr].b.val;
                    self.core[bptr].b.val = sum;
                },
                _  => panic!("Modifier not implemented"),
            },
            JMP => return Some(aptr),
            JMZ => match cell.modifier {
                A | BA    if self.core[bptr].a.val == 0 => return Some(aptr),
                B | AB    if self.core[bptr].b.val == 0 => return Some(aptr),
                F | X | I if self.core[bptr].a.val == 0
                          && self.core[bptr].b.val == 0 => return Some(aptr),
                _                                       => { },
            },
            JMN => match cell.modifier {
                A | BA    if self.core[bptr].a.val != 0 => return Some(aptr),
                B | AB    if self.core[bptr].b.val != 0 => return Some(aptr),
                F | X | I if self.core[bptr].a.val != 0
                          && self.core[bptr].b.val != 0 => return Some(aptr),
                _                                       => { },
            },
            CMP => match cell.modifier {
                A  if self.core[aptr].a.val == self.core[bptr].a.val => return Some(self.add_iptr(iptr, 2)),
                B  if self.core[aptr].b.val == self.core[bptr].b.val => return Some(self.add_iptr(iptr, 2)),
                AB if self.core[aptr].a.val == self.core[bptr].b.val => return Some(self.add_iptr(iptr, 2)),
                BA if self.core[aptr].b.val == self.core[bptr].a.val => return Some(self.add_iptr(iptr, 2)),
                F  if self.core[aptr].a.val == self.core[bptr].a.val
                   && self.core[aptr].b.val == self.core[bptr].b.val => return Some(self.add_iptr(iptr, 2)),
                X  if self.core[aptr].a.val == self.core[bptr].b.val
                   && self.core[aptr].b.val == self.core[bptr].a.val => return Some(self.add_iptr(iptr, 2)),
                I  if self.core[aptr] == self.core[bptr]             => return Some(self.add_iptr(iptr, 2)),
                _                                                    => { },
            },
            _   => panic!("Instruction not implemented"),
        }

        Some(self.add_iptr(iptr, 1))
    }

    fn add_iptr(&self, iptr: usize, add: i32) -> usize {
        (iptr as i32 + add + self.core.len() as i32) as usize % self.core.len()
    }
}

fn main() {
    let mut mars = Mars::new(32);
    // // Imp
    // mars.load(vec![
    //     Cell::new(MOV, None, Param::new(Direct, 0), Param::new(Direct, 1)),
    // ]);
    // Dwarf
    mars.load(vec![
        Cell::new(ADD, None, Param::new(Immediate, 4), Param::new(Direct, 3)),
        Cell::new(MOV, None, Param::new(Direct, 2), Param::new(Indirect, 2)),
        Cell::new(JMP, None, Param::new(Direct, -2), Param::new(Immediate, 0)),
        Cell::new(DAT, None, Param::new(Immediate, 0), Param::new(Immediate, 0))
    ]);
    mars.run();
}
