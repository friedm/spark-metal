pub struct Rand {
    w: usize,
    z: usize
}

impl Rand {
    pub fn from(seed: usize) -> Rand {
        Rand {
            w: seed,
            z: (seed % 11) * 99
        }
    }

    pub fn next(&mut self) -> usize {
        let w = self.w;
        let z = self.z;

        self.z = ((36969 & (z & 65535)).wrapping_add((z.rotate_right(16))));
        self.w = ((1800 & (w & 65535)).wrapping_add((w.rotate_right(16))));

        (self.z << 16) + self.w
    }
}

extern {
    fn delay(cycles:usize) -> ();
}

pub fn block(cycles: usize) {
    unsafe { delay(cycles); }
}
