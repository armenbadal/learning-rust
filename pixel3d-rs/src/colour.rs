use std::fmt::Display;

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Colour {
    // TODO 7.2: Ավելացնել գունային ալիքների հասանելիություն և f32 ներկայացում։
    // Ինտերպոլացնել float-երով, ապա վերջնական գույնը սահմանափակել ու կլորացնել u8-ի։
    // Ստուգել ալիքների սահմանները և հայտնի կշիռներով գույների խառնուրդը։
    pub const BLACK: Self = Self{red:0,green:0,blue:0,alpha:255};

    pub const fn new(red:u8, green:u8, blue:u8) -> Self{
        Self{red, green, blue, alpha:255}
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
