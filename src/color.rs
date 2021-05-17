use rand::{thread_rng, Rng};

pub enum Color {
    RED,
    BLUE,
    GREEN,
    RGB(u8, u8, u8)
}

impl Color {
    pub fn random() -> Color {
        let mut rng = thread_rng();
        Color::RGB(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
    }
}
