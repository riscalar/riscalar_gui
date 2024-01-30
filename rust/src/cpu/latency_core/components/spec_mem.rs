use crate::cpu::{exception::Exception, latency_core::LatencyCore};

impl LatencyCore {
    pub fn spec_read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match size {
            8 => self.spec_read8(addr),
            16 => self.spec_read16(addr),
            32 => self.spec_read32(addr),
            64 => self.spec_read64(addr),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    pub fn spec_write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match size {
            8 => self.spec_write8(addr, value),
            16 => self.spec_write16(addr, value),
            32 => self.spec_write32(addr, value),
            64 => self.spec_write64(addr, value),
            _ => return Err(Exception::StoreAMOAccessFault),
        }
        Ok(())
    }

    pub fn spec_mem_clear(&mut self) {
        self.spec_mem.clear();
    }

    fn spec_read8(&self, addr: u64) -> Result<u64, Exception> {
        match self.spec_mem.get(&addr) {
            Some(val) => Ok(*val as u64),
            None => self.memory.read(addr, 8),
        }
    }

    fn spec_read16(&self, addr: u64) -> Result<u64, Exception> {
        let low = self.spec_read8(addr)?;
        let high = self.spec_read8(addr + 1)?;
        Ok(low | (high << 8))
    }

    fn spec_read32(&self, addr: u64) -> Result<u64, Exception> {
        let low = self.spec_read16(addr)?;
        let high = self.spec_read16(addr + 2)?;
        Ok(low | (high << 16))
    }

    fn spec_read64(&self, addr: u64) -> Result<u64, Exception> {
        let low = self.spec_read32(addr)?;
        let high = self.spec_read32(addr + 4)?;
        Ok(low | (high << 32))
    }

    fn spec_write8(&mut self, addr: u64, val: u64) {
        self.spec_mem.insert(addr, val as u8);
    }

    fn spec_write16(&mut self, addr: u64, val: u64) {
        self.spec_mem.insert(addr, val as u8);
        self.spec_mem.insert(addr + 1, (val >> 8) as u8);
    }

    fn spec_write32(&mut self, addr: u64, val: u64) {
        self.spec_mem.insert(addr, val as u8);
        self.spec_mem.insert(addr + 1, (val >> 8) as u8);
        self.spec_mem.insert(addr + 2, (val >> 16) as u8);
        self.spec_mem.insert(addr + 3, (val >> 24) as u8);
    }

    fn spec_write64(&mut self, addr: u64, val: u64) {
        self.spec_mem.insert(addr, val as u8);
        self.spec_mem.insert(addr + 1, (val >> 8) as u8);
        self.spec_mem.insert(addr + 2, (val >> 16) as u8);
        self.spec_mem.insert(addr + 3, (val >> 24) as u8);
        self.spec_mem.insert(addr + 4, (val >> 32) as u8);
        self.spec_mem.insert(addr + 5, (val >> 40) as u8);
        self.spec_mem.insert(addr + 6, (val >> 48) as u8);
        self.spec_mem.insert(addr + 7, (val >> 56) as u8);
    }
}
