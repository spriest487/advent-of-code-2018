use {
    std::collections::{
        HashMap,
        HashSet,
    }
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,
    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

const OPS: [Opcode; 16] = [
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
];

use self::Opcode::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Device {
    registers: [usize; 4]
}

impl Device {
    fn new() -> Self {
        Self { registers: [0; 4] }
    }

    fn load(&self, reg: usize) -> usize {
        self.registers[reg]
    }

    fn store(&mut self, reg: usize, val: usize) {
        self.registers[reg] = val;
    }

    fn execute(&mut self, op: Opcode, a: usize, b: usize, c: usize) {
        match op {
            Addr => self.store(c, self.load(a) + self.load(b)),
            Addi => self.store(c, self.load(a) + b),

            Borr => self.store(c, self.load(a) | self.load(b)),
            Bori => self.store(c, self.load(a) | b),

            Banr => self.store(c, self.load(a) & self.load(b)),
            Bani => self.store(c, self.load(a) & b),

            Mulr => self.store(c, self.load(a) * self.load(b)),
            Muli => self.store(c, self.load(a) * b),

            Setr => self.store(c, self.load(a)),
            Seti => self.store(c, a),

            Gtir => self.store(c, if a > self.load(b) { 1 } else { 0 }),
            Gtri => self.store(c, if self.load(a) > b { 1 } else { 0 }),
            Gtrr => self.store(c, if self.load(a) > self.load(b) { 1 } else { 0 }),

            Eqir => self.store(c, if a == self.load(b) { 1 } else { 0 }),
            Eqri => self.store(c, if self.load(a) == b { 1 } else { 0 }),
            Eqrr => self.store(c, if self.load(a) == self.load(b) { 1 } else { 0 }),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    code: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
struct SampleOperation {
    before: Device,
    instruction: Instruction,
    after: Device,
}

impl SampleOperation {
    fn matches_op(&self, op: Opcode) -> bool {
        let mut device = self.before.clone();
        device.execute(op, self.instruction.a, self.instruction.b, self.instruction.c);
        device == self.after
    }
}

fn parse_4(s: &str, sep: char) -> (usize, usize, usize, usize) {
    let mut parts = s.split(sep);
    let a = parts.next().unwrap().trim().parse().unwrap();
    let b = parts.next().unwrap().trim().parse().unwrap();
    let c = parts.next().unwrap().trim().parse().unwrap();
    let d = parts.next().unwrap().trim().parse().unwrap();
    (a, b, c, d)
}

fn parse_device_sample(s: &str) -> Device {
    let (r0, r1, r2, r3) = parse_4(s, ',');
    Device { registers: [r0, r1, r2, r3] }
}

fn parse_instruction(s: &str) -> Instruction {
    let (code, a, b, c) = parse_4(s, ' ');
    Instruction { code, a, b, c }
}

fn parse_input(s: &str) -> (Vec<SampleOperation>, Vec<Instruction>) {
    let mut lines = s.lines();
    let mut sample_ops = Vec::new();
    let mut program = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.starts_with("Before") {
            let before = parse_device_sample(&line["Before: [".len()..line.len() - 1]);

            let instruction_line = lines.next().unwrap();
            let instruction = parse_instruction(&instruction_line);

            let after_line = lines.next().unwrap();
            let after = parse_device_sample(&after_line["After:  [".len()..after_line.len() - 1]);

            // ends with either a blank line or eof
            if let Some(last) = lines.next() {
                assert!(last.chars().all(|c| char::is_ascii_whitespace(&c)));
            }

            sample_ops.push(SampleOperation {
                before,
                instruction,
                after,
            })
        } else {
            break
        }
    }

    // 2 blank lines between samples and program
    let lines = lines.skip(1);
    for line in lines {
        program.push(parse_instruction(line))
    }

    (sample_ops, program)
}

fn main() {
    let input = include_str!("day_16.txt");
    let (samples, program) = parse_input(input);

    let samples_matches: Vec<Vec<_>> = samples.iter()
        .map(|sample| {
            OPS.iter().filter(|op| sample.matches_op(**op))
                .collect()
        })
        .collect();

    println!("samples matching 3 or more ops: {}", samples_matches.iter()
        .filter(|matches| matches.len() >= 3)
        .count());

    let mut possible_codes_by_op = HashMap::new();
    for (sample_index, sample_matches) in samples_matches.iter().enumerate() {
        for &matched_op in sample_matches {
            let codes = possible_codes_by_op.entry(matched_op).or_insert_with(|| HashSet::new());
            let sample = &samples[sample_index];

            codes.insert(sample.instruction.code);
        }
    }

    let mut op_codes = HashMap::new();
    let mut samples = samples;

    // an op/code combo only makes if all the samples with that code match the op. find the first
    // correct combo and remove it until there's none left, dealing with possible ambiguous combos
    // by process of elimination
    while !possible_codes_by_op.is_empty() {
        let (op, code) = possible_codes_by_op.iter()
            .filter_map(|(op, possible_codes)| {
                let codes: Vec<_> = possible_codes.iter()
                    .filter(|code| {
                        samples.iter().filter(|s| s.instruction.code == **code)
                            .all(|s| s.matches_op(**op))
                    })
                    .collect();

                if codes.len() == 1 {
                    println!("op {:?} is code {}", op, codes[0]);
                    Some((**op, *codes[0]))
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        op_codes.insert(code, op);
        possible_codes_by_op.remove(&op);

        // no other ops will have this code, so remove it as a possibility
        for other_op_codes in possible_codes_by_op.values_mut() {
            other_op_codes.remove(&code);
        }

        samples.retain(|s| s.instruction.code != code);
    }

    // now we know the opcodes, we can execute the program
    let mut device = Device::new();
    for instruction in program {
        let op = op_codes[&instruction.code];
        device.execute(op, instruction.a, instruction.b, instruction.c);
        println!("executing: {:?} {:2} {:2} {:2} -> {:4?}",
            op, instruction.a, instruction.b, instruction.c,
            device.registers);
    }

    println!("register 0 at end of execution is: {}", device.registers[0]);
}
