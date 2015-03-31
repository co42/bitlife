use Address::{ Immediate, Direct, Indirect, PreIncIndirect, PostIncIndirect };
use Instruction::{ DAT, MOV, ADD, SUB, MUL, DIV, MOD, JMP, JMZ, JMN, DJN, CMP, SLT, SPL };
use Modifier::{ A, B, AB, BA, F, X, I };

// Address modes
enum Address {
    Immediate,
    Direct,
    Indirect,
    PreIncIndirect,
    PostIncIndirect,
}

// Instructions
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
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

struct Param {
    address: Address,
    value:   i16,
}

struct Mars {
    core: Vec<(Instruction, Modifier, Param, Param)>,
}

impl Mars {
    fn new() -> Mars {
        Mars { core: vec![] }
    }

    fn execute(&mut self, iptr: usize) {
        match &self.core[iptr] {
            &(MOV, ref modifier, ref a, ref b) => { },
            _                                  => { },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
