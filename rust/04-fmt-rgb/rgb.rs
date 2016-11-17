use std::fmt;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_red = format!("{:#X}", self.red);

        write!(f, "RGB ({}, {}, {})", 
               self.red, 
               self.green,
               self.blue);
        write!(f, "{:2}", formatted_red)
    }
}


fn main() {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 }
    ].iter() {
        println!("{}", *color)
    }
}

// OUTPUT
// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000
