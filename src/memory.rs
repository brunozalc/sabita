// 65536 bytes of memory
// 000h - 0FFh |> RST and interrupts config
// 100h - 14Fh |> ROM metadata (name, checksum, etc.)
// 150h - 7FFFh |> ROM space (cartridge)
// 8000h - 9FFFh |> VRAM (video RAM, for the display)
// A000h - BFFFh |> external space RAM
// C000h - DFFFh |> working RAM (WRAM)
// FE00h - FFFFh |> internal RAM (IRAM), which includes OAM and the stack

pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
