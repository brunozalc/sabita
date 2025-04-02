macro_rules! get_set {
    ($reg:ident, $get_name:ident, $set_name:ident, $size:ty) => {
        pub fn $get_name(&self) -> $size {
            self.$reg
        }

        pub fn $set_name(&mut self, value: $size) {
            self.$reg = value;
        }
    };
}

macro_rules! get_set_pair {
    ($reg_hi:ident, $reg_lo:ident, $get_name:ident, $set_name:ident) => {
        pub fn $get_name(&self) -> u16 {
            ((self.$reg_hi as u16) << 8) | (self.$reg_lo as u16)
        }

        pub fn $set_name(&mut self, value: u16) {
            self.$reg_hi = (value >> 8) as u8; // stores the highest 8 bits from the original value
            self.$reg_lo = value as u8; // because we're truncating the value to 8 bits, it
            // will only store the lowest 8 bits from the original value
        }
    };
}

pub struct Registers {
    a: u8,
    f: u8, // Z: zero, N: subtract, H: half-carry, C: carry
    b: u8,
    c: u8, // b: hi, c: lo
    d: u8,
    e: u8, // d: hi, e: lo
    h: u8,
    l: u8, // h: hi, l: lo
    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0x150, // user program starts at 0x150. refer to memory.rs
            pc: 0,
        }
    }

    get_set!(a, get_a, set_a, u8);
    get_set!(b, get_b, set_b, u8);
    get_set!(c, get_c, set_c, u8);
    get_set!(d, get_d, set_d, u8);
    get_set!(e, get_e, set_e, u8);
    get_set!(h, get_h, set_h, u8);
    get_set!(l, get_l, set_l, u8);
    get_set!(sp, get_sp, set_sp, u16);
    get_set!(pc, get_pc, set_pc, u16);

    get_set_pair!(b, c, get_bc, set_bc);
    get_set_pair!(d, e, get_de, set_de);
    get_set_pair!(h, l, get_hl, set_hl);

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0; // only the highest 4 bits are used for flags
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8; 
    }
    
}
