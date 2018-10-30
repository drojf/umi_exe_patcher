
use byteorder::{ByteOrder, LittleEndian};

pub struct ScreenResolution {
    pub width : u32,
    pub height : u32,
}

impl ScreenResolution {
    pub fn new(width : u32, height : u32) -> ScreenResolution {
        ScreenResolution {
            width,
            height,
        }
    }
}

pub trait GetDimensionsSearchString {
    fn get_height_bytes(height: u32) -> Vec<u8>;
    fn get_width_bytes(width: u32) -> Vec<u8>;
}

pub struct DimensionsWindows {}

impl DimensionsWindows {
    fn get_dimensions_search_string(magic: u8, value: u32) -> Vec<u8>
    {
        let mut bytes: [u8; 5] = [magic, 0, 0, 0, 0];
        LittleEndian::write_u32(&mut bytes[1..5], value);
        bytes.to_vec()
    }
}

impl GetDimensionsSearchString for DimensionsWindows {
    fn get_height_bytes(height: u32) -> Vec<u8>
    {
        DimensionsWindows::get_dimensions_search_string(186, height)
    }

    fn get_width_bytes(width: u32) -> Vec<u8>
    {
        DimensionsWindows::get_dimensions_search_string(185, width)
    }
}

pub struct DimensionsMac {}

impl DimensionsMac {
    pub fn get_dimensions_search_string(magic1: u8, magic2:u8, value: u16) -> Vec<u8>
    {
        let mut bytes: [u8; 4] = [magic1, magic2, 0, 0];
        LittleEndian::write_u16(&mut bytes[2..4], value);
        bytes.to_vec()
    }
}

impl GetDimensionsSearchString for DimensionsMac
{
    fn get_height_bytes(height: u32) -> Vec<u8>
    {
        DimensionsMac::get_dimensions_search_string(0x66, 0xb9, height as u16)
    }

    fn get_width_bytes(width: u32) -> Vec<u8>
    {
        DimensionsMac::get_dimensions_search_string(0x66, 0xb8, width as u16)
    }
}


