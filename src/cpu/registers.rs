pub struct Registers {
    a: u8, f: u8, // accumulator and flags registers
    b: u8, c: u8, // b: hi, c: lo
    d: u8, e: u8, // d: hi, e: lo
    h: u8, l: u8, // h: hi, l: lo
    sp: u16, pc: u16
}


impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0, pc: 0
        }
    }
}
