use wasm_bindgen::prelude::*;

/// tiny xorshift32 RNG (no std / no getrandom)
#[wasm_bindgen]
pub struct AntWar {
    width: u32,
    height: u32,
    buf: Vec<u8>,
    state: u32,
}

#[wasm_bindgen]
impl AntWar {
    /// Create with initial size (will be resized by JS to fit screen).
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> AntWar {
        let len = (width * height * 4) as usize;
        AntWar {
            width,
            height,
            buf: vec![0; len],
            state: 0x1234_abcd, // seed; can be randomized from JS if desired
        }
    }

    /// Resize the pixel buffer to match canvas.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.buf.resize((width * height * 4) as usize, 0);
    }

    /// Pointer to pixel buffer (RGBA8).
    pub fn pixels_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    /// Current width/height for JS convenience.
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    /// Fill buffer with fresh static (grayscale noise).
    pub fn render(&mut self) {
        // fill each pixel with random gray, alpha=255
        let mut i = 0usize;
        let total = (self.width * self.height) as usize;
        while i < total {
            let v = self.next_u8();
            let base = i * 4;
            self.buf[base] = v;       // R
            self.buf[base + 1] = v;   // G
            self.buf[base + 2] = v;   // B
            self.buf[base + 3] = 255; // A
            i += 1;
        }
    }
}

impl AntWar {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        // xorshift32
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
    #[inline(always)]
    fn next_u8(&mut self) -> u8 {
        (self.next_u32() & 0xFF) as u8
    }
}
