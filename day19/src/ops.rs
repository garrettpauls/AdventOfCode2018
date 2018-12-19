use std::collections::HashMap;

pub type Reg = i64;
pub type Param = usize;

pub trait Operation {
    fn name(&self) -> &'static str;
    fn exec(&self, registers: &mut Vec<Reg>);
}

pub fn load_operations() -> HashMap<&'static str, &'static Fn(&mut Vec<Reg>, Param, Param, Param)> {
    let mut m: HashMap<&'static str, &'static Fn(&mut Vec<Reg>, Param, Param, Param)> = HashMap::new();
    m.insert("addr", &addr);
    m.insert("addi", &addi);
    m.insert("mulr", &mulr);
    m.insert("muli", &muli);
    m.insert("banr", &banr);
    m.insert("bani", &bani);
    m.insert("borr", &borr);
    m.insert("bori", &bori);
    m.insert("setr", &setr);
    m.insert("seti", &seti);
    m.insert("gtir", &gtir);
    m.insert("gtri", &gtri);
    m.insert("gtrr", &gtrr);
    m.insert("eqir", &eqir);
    m.insert("eqri", &eqri);
    m.insert("eqrr", &eqrr);
    m
}

pub fn addr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") + registers.get(b).expect("reg b")
    }
}

pub fn addi(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") + b as Reg
    }
}

pub fn mulr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") * registers.get(b).expect("reg b")
    }
}

pub fn muli(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") * b as Reg
    }
}

pub fn banr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") & registers.get(b).expect("reg b")
    }
}

pub fn bani(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") & b as Reg
    }
}

pub fn borr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") | registers.get(b).expect("reg b")
    }
}

pub fn bori(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        registers.get(a).expect("reg a") | b as Reg
    }
}

pub fn setr(registers: &mut Vec<Reg>, a: Param, _: Param, c: Param) {
    registers[c] = {
        *registers.get(a).expect("reg a")
    }
}

pub fn seti(registers: &mut Vec<Reg>, a: Param, _: Param, c: Param) {
    registers[c] = {
        a as Reg
    }
}

pub fn gtir(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if a as Reg > *registers.get(b).expect("reg b") {
            1
        } else {
            0
        }
    }
}

pub fn gtri(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if *registers.get(a).expect("reg a") > b as Reg {
            1
        } else {
            0
        }
    }
}

pub fn gtrr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if registers.get(a).expect("reg a") > registers.get(b).expect("reg b") {
            1
        } else {
            0
        }
    }
}

pub fn eqir(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if a as Reg == *registers.get(b).expect("reg b") {
            1
        } else {
            0
        }
    }
}

pub fn eqri(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if *registers.get(a).expect("reg a") == b as Reg {
            1
        } else {
            0
        }
    }
}

pub fn eqrr(registers: &mut Vec<Reg>, a: Param, b: Param, c: Param) {
    registers[c] = {
        if registers.get(a).expect("reg a") == registers.get(b).expect("reg b") {
            1
        } else {
            0
        }
    }
}
