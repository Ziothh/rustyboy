# Hardware
NOTE: I'm only analysing the Game Boy Classic

## Architecture
`16-bit` (`64kB`) address bus connection between regions

 - DMG-01 SoC
    - CPU
       - HRAM: `127B` (`0xFF80..=0xFFFE`) (fast `LD` with `LDH` instruction)
    - PPU
       - OAM: `160B` (`0xFE00..=0xFE9F`)
       - DMA
       - Palette
       - registers
 - Cartridge
    - ROM `32kB` (`0x0000..=0x7FFF`)
      - bank 00:    `16kB` (`0x0000..=0x3FFF`)
      - bank 01~NN: `16kB` (`0x4000..=0x7FFF`) (switchable)
    - External RAM `8kB` (`0xA000..=0zBFFF`)
  - RAM
    - WRAM `8kB` (`0xC000..=0xDFFF`) (last `4kB` switchable in CGB mode)
    - VRAM `8kB` (`0x8000..=0x9FFF`) (switchable in CGB mode)
        - Tile data
        - Tile maps
  - IO registers (`0xFF40..=0xFF4B`)

### Useful links
[Game Boy Architecture](https://www.copetti.org/writings/consoles/game-boy/)
[Game Boy Color system design](https://course.ece.cmu.edu/~ece545/F16/reports/F14_GameboyColor.pdf)

## CPU
 - Model: DMG-01 (~= ZiLOG Z80)
 - Clock: 4.16MHz (acts like 1MHz)
 - 16-bit

 - Registers (memory cells, directly integrated in the CPU)
    - 8-bit
        - B C
        - D E
        - H L
        - A F(lags)
    - 16-bit
        - SP (Stack Pointer)
        - PC (Program Counter)


## Memory
 - 8kb work RAM
 - 8kb VRAM

## ROM
 - [bootrom repo](https://github.com/ISSOtm/gb-bootroms) (TODO: CHECK THIS OUT)
 - 
