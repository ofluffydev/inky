use spin::Mutex;

pub struct Paint<'a> {
    pub image: &'a mut [u8; 4006],
    pub width: usize,
    pub height: usize,
    pub width_memory: usize,
    pub height_memory: usize,
    pub color: u8,
    pub rotate: Rotation,
    pub mirror: Mirror,
    pub width_byte: usize,
    pub height_byte: usize,
    pub scale: usize,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Clone, Copy)]
pub enum Mirror {
    None,
    Horizontal,
    Vertical,
    Both,
}

pub const G_IMAGE_2IN13_2: [u8; 4006] = [ /* 0x00,0x01,0x7A,0x00,0xFA,0x00, */
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0E, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x03, 0xF0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xF0, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x04, 0x1E, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xF0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xBF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x38, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xE0, 0x1F, 0xCF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x0F, 0x9F, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x31, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x07, 0x9F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0x33, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0x87, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0x86, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x07, 0xFF, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x86, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x03, 0xFF, 0x80, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x84, 0x3F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x80, 0x7F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x80, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x80, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x07, 0xFF, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x81, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x83, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x83, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xCF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x87, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3E, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x87, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1C, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0E, 0x00, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xF0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xFF, 0xE0, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xFF, 0xE0, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xFF, 0xE0, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xFF, 0xE0, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x7F, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x07, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3F, 0xFF, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x70, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x70, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x83, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x70, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x70, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xE0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x38, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xF0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1C, 0x3C, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x3F, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x1F, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0C, 0x0F, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x07, 0x80, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x01, 0xFF, 0x80, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x03, 0xFF, 0xC0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0xE7, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x81, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x80, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x3F, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xAF, 0x80, 0x7F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0x00, 0x3F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0E, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0E, 0x1E, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0E, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x07, 0xF0, 0x60, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x84, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xF0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x7F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xF0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x7F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3E, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x38, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x38, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x30, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0x33, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x03, 0xFF, 0x80, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x01, 0xFE, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0C, 0x3F, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x01, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x07, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x81, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x7F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xF0, 0x1F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x07, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0x7F, 0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xF0, 0x1F, 0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFC, 0x0F, 0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xF0, 0x1F, 0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0x7F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x07, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x08, 0x00, 0x60, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1C, 0x00, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xE0, 0x7F, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x81, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x07, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x3F, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xE0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0x83, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC1, 0x83, 0x9F, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x03, 0xFF, 0x80, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x01, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x0F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x8F, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3F, 0x80, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x87, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x83, 0xC3, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xE0, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x00, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x0F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x3F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x1F, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x1F, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x1F, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x0E, 0x1F, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x00, 0x70, 0x3F, 0x80, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x01, 0xFC, 0x7F, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x03, 0xFE, 0x7F, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0xFF, 0xE0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0x87, 0xE0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F, 0xFF, 0xF8, 0x00, 0x0F, 0x03, 0xC0, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0x1F, 0xFF, 0xF8, 0x00, 0x0F, 0x03, 0xC0, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0x1F, 0xFF, 0xF8, 0x00, 0x0F, 0x03, 0xC0, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x1F, 0xFF, 0xF8, 0x00, 0x07, 0x03, 0xC0, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x3F, 0xFF, 0xF8, 0x00, 0x07, 0x80, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x7F, 0xFF, 0xF8, 0x00, 0x07, 0x80, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x03, 0xC0, 0x01, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x03, 0x80, 0x01, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFC, 0x00, 0x00, 0x0F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xC0, 0x00, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFC, 0x00, 0x1C, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xF8, 0x00, 0x7E, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xE0, 0x01, 0xFF, 0x80, 0x07, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xC0, 0x07, 0xFF, 0xF8, 0x07, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xC0, 0x1F, 0xFF, 0xFC, 0x07, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0x80, 0x3F, 0xE1, 0xFC, 0x07, 0xF8, 0x00, 0x07, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0x00, 0x3F, 0xC0, 0x7E, 0x07, 0xF8, 0x00, 0x03, 0x80, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFE, 0x00, 0x3F, 0xF0, 0x7F, 0x07, 0xF8, 0x00, 0x03, 0x80, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFE, 0x00, 0x1F, 0xFC, 0x00, 0x07, 0xF8, 0x00, 0x03, 0x80, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFE, 0x00, 0x07, 0xFF, 0x00, 0x07, 0xF8, 0x00, 0x03, 0x80, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFC, 0x00, 0x00, 0xFF, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFC, 0x1F, 0xE0, 0x3F, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFC, 0x0F, 0xF0, 0x3F, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x07, 0xFC, 0xEF, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x03, 0xFF, 0x98, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x01, 0xFF, 0xF0, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x9F, 0xFC, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x07, 0xFF, 0x00, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0xFF, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0xFF, 0x80, 0x07, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x07, 0xFF, 0x80, 0x0F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xFC, 0x00, 0x0F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xF0, 0x00, 0x0F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xC0, 0x00, 0x0F, 0xF8, 0x00, 0x00, 0xFE, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xF8, 0x00, 0x1F, 0xF8, 0x00, 0x03, 0xFF, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x07, 0xFE, 0x00, 0x1F, 0xF8, 0x00, 0x03, 0xFF, 0xE0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0xFF, 0x80, 0x3F, 0xF8, 0x00, 0x07, 0xFF, 0xF0, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x01, 0xFF, 0x80, 0x3F, 0xF8, 0x00, 0x07, 0x81, 0xF8, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x1F, 0xFF, 0x80, 0x7F, 0xF8, 0x00, 0x0F, 0x00, 0x7E, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xFF, 0x00, 0xFF, 0xF8, 0x00, 0x0F, 0x00, 0x3F, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xF8, 0x01, 0xFF, 0xF8, 0x00, 0x0F, 0x00, 0x1F, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3F, 0xC0, 0x01, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0x0F, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x3E, 0x00, 0x07, 0xFF, 0xF8, 0x00, 0x07, 0x00, 0x07, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x30, 0x00, 0x0F, 0xFF, 0xF8, 0x00, 0x07, 0x80, 0x03, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0x00, 0x3F, 0xFF, 0xF8, 0x00, 0x03, 0xC0, 0x01, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xF8, 0x00, 0x03, 0xC0, 0x00, 0xF0, 0x00, 0x00, 0x00,
    0xFF, 0xF8, 0x00, 0x00, 0x07, 0xFF, 0xFF, 0xF8, 0x00, 0x01, 0x80, 0x00, 0x70, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

    // 6 padding 0x00
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

lazy_static::lazy_static! {
    static ref IMAGE_MUTEX: Mutex<[u8; 4000]> = Mutex::new([0xFF; 4000]); // Initialize with white (0xFF)
}

impl<'a> Paint<'a> {
    pub fn new_image(
        image: &'a mut [u8; 4006],
        width: usize,
        height: usize,
        rotate: Rotation,
        color: u8,
    ) -> Self {
        let width_byte = if width % 8 == 0 {
            width / 8
        } else {
            width / 8 + 1
        };

        let (view_width, view_height) = match rotate {
            Rotation::Rotate0 | Rotation::Rotate180 => (width, height),
            Rotation::Rotate90 | Rotation::Rotate270 => (height, width),
        };

        Paint {
            image,
            width: view_width,
            height: view_height,
            width_memory: width,
            height_memory: height,
            color,
            rotate,
            mirror: Mirror::None,
            width_byte,
            height_byte: height,
            scale: 1, // you can implement 2x scaling if needed
        }
    }

    pub fn get_image() -> &'static mut [u8; 4006] {
        let mut image = IMAGE_MUTEX.lock();
        // SAFETY: This is safe because the Mutex ensures exclusive access.
        unsafe { &mut *(image.as_mut_ptr() as *mut [u8; 4006]) }
    }
}
