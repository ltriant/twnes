mod mapper0;
mod mapper1;

use std::convert::From;

pub use mapper0::Mapper0;
pub use mapper1::Mapper1;

enum MirrorMode {
    Horizontal = 0,
    Vertical   = 1,
    Single0    = 2,
    Single1    = 3,
    Four       = 4,
}

impl MirrorMode {
    fn lookup(&self) -> Vec<usize> {
        match *self {
            MirrorMode::Horizontal => vec![0, 0, 1, 1],
            MirrorMode::Vertical   => vec![0, 1, 0, 1],
            MirrorMode::Single0    => vec![0, 0, 0, 0],
            MirrorMode::Single1    => vec![1, 1, 1, 1],
            MirrorMode::Four       => vec![0, 1, 2, 3],
        }
    }
}

impl From<u8> for MirrorMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => MirrorMode::Horizontal,
            1 => MirrorMode::Vertical,
            2 => MirrorMode::Single0,
            3 => MirrorMode::Single1,
            4 => MirrorMode::Four,
            _ => panic!("bad mirror mode: {}", mode)
        }
    }
}

pub trait Mapper {
    fn nametable_offset(&self, _table: usize) -> usize { 0 }

    fn read(&mut self, address: u16) -> Result<u8, String>;
    fn write(&mut self, address: u16, val: u8) -> Result<u8, String>;

    fn step(&mut self) {}
}


//
// This is an empty mapper that implements the Mapper trait, because I need to
// initialise the mapper to _something_ when I create the Console object.
//

pub struct MapperEmpty;
impl Mapper for MapperEmpty {
    fn read(&mut self, _address: u16) -> Result<u8, String> { Ok(0) }
    fn write(&mut self, _address: u16, _val: u8) -> Result<u8, String> { Ok(0) }
}
