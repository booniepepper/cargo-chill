pub static BANNER: &[u8] = include_bytes!("../art/banner.txt");
pub static CRATELIN: &[u8] = include_bytes!("../art/bots/cratelin.txt");
pub static _EZ11: &[u8] = include_bytes!("../art/bots/ez11.txt");

#[derive(Debug, Default)]
pub enum CoolNumber {
    #[default]
    Not,

    Overflow,
    Underflow,
    Perfect,
    Square(u8),
    Cube(u8),
    HyperCube(u8, u8),
    // TODO: Can I source more interesting sequences? (See: https://oeis.org, Combo Class, etc)
}

impl CoolNumber {
    pub fn coolness(from: u8, to: u8) -> Self {
        use CoolNumber::*;

        match (from, to) {
            (0, 255) => Underflow,
            (255, 0) => Overflow,
            (_, 6) | (_, 28) => Perfect,
            // TODO: Pre-calculate?
            _ => match to {
                4 => Square(2),
                8 => Cube(2),
                16 => HyperCube(2, 4),
                32 => HyperCube(2, 5),
                64 => HyperCube(2, 6),
                128 => HyperCube(2, 7),

                9 => Square(3),
                27 => Cube(3),
                81 => HyperCube(3, 4),
                243 => HyperCube(3, 5),

                25 => Square(5),
                125 => Cube(5),

                36 => Square(6),
                216 => Cube(6),

                49 => Square(7),
                100 => Square(10),
                121 => Square(11),
                144 => Square(12),
                169 => Square(13),
                196 => Square(14),
                225 => Square(15),
                _ => Not,
            },
        }
    }
}
