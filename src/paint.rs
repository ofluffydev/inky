use spin::Mutex;

#[repr(C)]
pub struct Paint<'a> {
    pub image: &'a mut [u8; 4000],
    pub width: usize,
    pub height: usize,
    pub width_memory: usize,
    pub height_memory: usize,
    pub color: u8,
    pub rotate: Rotation,
    pub mirror: Mirror,
    pub width_byte: usize,
    pub height_byte: usize,
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

lazy_static::lazy_static! {
    static ref IMAGE_MUTEX: Mutex<[u8; 4006]> = Mutex::new([0xFF; 4006]); // Initialize with white (0xFF)
    static ref IMAGE_MUTEX_BLACK: Mutex<[u8; 4006]> = Mutex::new([0x00; 4006]); // Initialize with white (0xFF)
}

impl<'a> Paint<'a> {
    pub fn new_image(
        image: &'a mut [u8; 4000],
        width: usize,
        height: usize,
        rotate: Rotation,
        color: u8,
    ) -> Self {
        let (view_width, view_height) = match rotate {
            Rotation::Rotate0 | Rotation::Rotate180 => (width, height),
            Rotation::Rotate90 | Rotation::Rotate270 => (height, width),
        };

        let width_byte = if width % 8 == 0 {
            width / 8
        } else {
            width / 8 + 1
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
        }
    }

    pub fn get_image() -> &'static mut [u8; 4000] {
        let mut image = IMAGE_MUTEX.lock();
        // SAFETY: This is safe because the Mutex ensures exclusive access.
        unsafe { &mut *(image.as_mut_ptr() as *mut [u8; 4000]) }
    }

    pub fn draw_bitmap(&mut self, image_buffer: &[u8]) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = x + y * self.width_byte;
                self.image[addr] = image_buffer[addr];
            }
        }
    }

    pub fn paint_clear_color(&mut self, color: u8) {
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = x + y * self.width_byte;
                self.image[addr] = color;
            }
        }
    }

    pub fn paint_black_screen(&mut self) {
        let black_image = IMAGE_MUTEX_BLACK.lock();
        for y in 0..self.height_byte {
            for x in 0..self.width_byte {
                let addr = x + y * self.width_byte;
                self.image[addr] = black_image[addr];
            }
        }
    }
}
