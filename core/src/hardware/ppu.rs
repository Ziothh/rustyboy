use super::bus;

pub struct PPU;
impl PPU {
    pub fn get_mode() -> PPUMode {
        todo!()
    }

    pub fn set_mode(mode: PPUMode, memory_bus: &mut bus::Interface) {
        // Unlock memory
        match PPU::get_mode() {
            PPUMode::Drawing => {memory_bus.unlock_region(bus::regions::VRAM);},
            PPUMode::OAMScan => {memory_bus.unlock_region(bus::regions::OAM);},
            _ => (), // Blanking modes don't block CPU from reading memory
        };

        // Lock memory
        match mode {
            PPUMode::Drawing => {memory_bus.lock_region(bus::regions::VRAM);},
            PPUMode::OAMScan => {memory_bus.lock_region(bus::regions::OAM);},
            _ => (), // Blanking modes don't block CPU from reading memory
        }
    }
}

pub enum PPUMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}
