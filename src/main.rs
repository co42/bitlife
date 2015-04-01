#![feature(thread_sleep)]
#![feature(std_misc)]

use std::thread::sleep;
use std::time::duration::Duration;

use Address::{ Immediate, Direct, Indirect, PreIncIndirect, PostIncIndirect };
use Instruction::{ DAT, MOV, ADD, SUB, MUL, DIV, MOD, JMP, JMZ, JMN, DJN, CMP, SLT, SPL };
use Modifier::{ A, B, AB, BA, F, X, I };

// Address modes
#[derive(Clone)]
enum Address {
    Immediate,       // #
    Direct,          // $
    Indirect,        //
    PreIncIndirect,  //
    PostIncIndirect, //
}

// Instructions
#[derive(Clone)]
enum Instruction {
    DAT,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JMP,
    JMZ,
    JMN,
    DJN,
    CMP,
    SLT,
    SPL,
}

// Modifiers
#[derive(Clone)]
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

#[derive(Clone)]
struct Param {
    addr: Address,
    val:  usize,
}

impl Param {
    fn new(addr: Address, val: usize) -> Param {
        Param { addr: addr, val: val }
    }
}

#[derive(Clone)]
struct Cell {
    ins:      Instruction,
    modifier: Modifier,
    a:        Param,
    b:        Param,
}

impl Cell {
    fn new(ins: Instruction, modifier: Modifier, a: Param, b: Param) -> Cell {
        Cell { ins: ins, modifier: modifier, a: a, b: b }
    }
}

struct Mars {
    core: Vec<Cell>,
    iptr: usize,
}

impl Mars {
    fn new(size: usize) -> Mars {
        let default = Cell::new(DAT, F, Param::new(Immediate, 0), Param::new(Immediate, 0));
        Mars { core: vec![default; size], iptr: 0 }
    }

    fn load(&mut self, warrior: Vec<Cell>) {
        self.core[0..warrior.len()] = warrior[..];
    }

    fn run(&mut self) {
        loop {
            let iptr = self.iptr;

            // Debug
            for (cptr, cell) in self.core.iter().enumerate() {
                if cptr == self.iptr {
                    print!("\x1B[0;31m");
                }
                print!("_");
                if cptr == self.iptr {
                    print!("\x1B[0;0m");
                }
            }
            println!("");

            self.iptr = self.execute(iptr);
            sleep(Duration::milliseconds(100));
        }
    }

    fn execute(&mut self, iptr: usize) -> usize {
        let cell = self.core[iptr].clone();

        // Parameters
        let aptr = match cell.a.addr {
            Immediate => 0,
            Direct    => cell.a.val,
            _         => panic!("Address not implemented"),
        };
        let bptr = match cell.b.addr {
            Immediate => 0,
            Direct => cell.b.val,
            _      => panic!("Address not implemented"),
        };

        // Instruction
        match cell.ins {
            DAT => { },
            MOV => {
                match cell.modifier {
                    I => {
                        let src = self.add_iptr(iptr, aptr);
                        let dst = self.add_iptr(iptr, bptr);
                        self.core[dst] = self.core[src].clone()
                    },
                    _ => panic!("Modifier not implemented"),
                }
            }
            _   => panic!("Instruction not implemented"),
        }

        self.add_iptr(iptr, 1)
    }

    fn add_iptr(&self, iptr: usize, add: usize) -> usize {
        (iptr + add) % self.core.len()
    }
}

fn main() {
    let mut mars = Mars::new(16);
    mars.load(vec![
        Cell::new(MOV, I, Param::new(Direct, 0), Param::new(Direct, 1))
    ]);
    mars.run();
    println!("Hello, world!");
}
