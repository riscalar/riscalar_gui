use crate::cpu::exception::Exception;

pub type Byte = u8;
pub const MEM_BASE: u64 = 0x0000_0000;
pub const MEM_SIZE: u64 = 1024 * 1024 * 1024;
const MEM_END: u64 = MEM_BASE + MEM_SIZE;

#[derive(Clone)]
pub struct Memory {
    pub data: Vec<Byte>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0; MEM_SIZE as usize],
        }
    }

    pub fn load_binary(&mut self, binary: Vec<Byte>) {
        self.data.splice(..binary.len(), binary.iter().cloned());
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match addr {
            MEM_BASE..=MEM_END => match size {
                8 => Ok(self.read8(addr)),
                16 => Ok(self.read16(addr)),
                32 => Ok(self.read32(addr)),
                64 => Ok(self.read64(addr)),
                _ => Err(Exception::LoadAccessFault),
            },
            _ => Err(Exception::LoadAccessFault),
        }
    }

    /// Store `size`-bit data to the dataory.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match addr {
            MEM_BASE..=MEM_END => match size {
                8 => self.write8(addr, value),
                16 => self.write16(addr, value),
                32 => self.write32(addr, value),
                64 => self.write64(addr, value),
                _ => return Err(Exception::StoreAMOAccessFault),
            },
            _ => return Err(Exception::StoreAMOAccessFault),
        }
        Ok(())
    }

    /// Write byte to dataory.
    fn write8(&mut self, addr: u64, val: u64) {
        let index = (addr - MEM_BASE) as usize;
        self.data[index] = val as u8
    }

    /// Write halfword to data.
    fn write16(&mut self, addr: u64, val: u64) {
        let index = (addr - MEM_BASE) as usize;
        self.data[index] = (val & 0xff) as u8;
        self.data[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    /// Write word to data.
    fn write32(&mut self, addr: u64, val: u64) {
        let index = (addr - MEM_BASE) as usize;
        for i in 0..=3 {
            self.data[index + i] = ((val >> (i * 8)) & 0xff) as u8;
        }
    }

    /// Write doubleword to data.
    fn write64(&mut self, addr: u64, val: u64) {
        let index = (addr - MEM_BASE) as usize;
        for i in 0..=7 {
            self.data[index + i] = ((val >> (i * 8)) & 0xff) as u8;
        }
    }

    /// Read byte from dataory.
    fn read8(&self, addr: u64) -> u64 {
        let index = (addr - MEM_BASE) as usize;
        self.data[index] as u64
    }

    /// Read halfword from data.
    fn read16(&self, addr: u64) -> u64 {
        let index = (addr - MEM_BASE) as usize;
        (self.data[index] as u64) | ((self.data[index + 1] as u64) << 8)
    }

    /// Read word from data.
    fn read32(&self, addr: u64) -> u64 {
        let index = (addr - MEM_BASE) as usize;
        (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24)
    }

    /// Read doubleword from data.
    fn read64(&self, addr: u64) -> u64 {
        let index = (addr - MEM_BASE) as usize;
        (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24)
            | ((self.data[index + 4] as u64) << 32)
            | ((self.data[index + 5] as u64) << 40)
            | ((self.data[index + 6] as u64) << 48)
            | ((self.data[index + 7] as u64) << 56)
    }

    // pub fn translate_address(&mut self, addr: MemoryAddress) {
    //     // Happens after page table first level miss
    //     self.ptbl_access += 1;
    //     self.ptbl_misses += 1;
    // }

    // pub fn new_page(&mut self, addr: MemoryAddress) {
    //     self.page_count += 1;
    // }

    // pub fn access(
    //     &mut self,
    //     addr: MemoryAddress,
    //     cmd: MemoryCommand,
    //     data: Option<u32>,
    // ) -> Option<InstructionFault> {
    //     // Happens after page table first level hit
    //     self.ptbl_access += 1;
    //     None
    // }
}
