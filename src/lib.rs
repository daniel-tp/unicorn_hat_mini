use rgb::RGB8;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use thiserror::Error;

const COLS: usize = 17;
const ROWS: usize = 7;

const LUT: [[usize; 3]; COLS * ROWS] = [
    [139, 138, 137],
    [223, 222, 221],
    [167, 166, 165],
    [195, 194, 193],
    [111, 110, 109],
    [55, 54, 53],
    [83, 82, 81],
    [136, 135, 134],
    [220, 219, 218],
    [164, 163, 162],
    [192, 191, 190],
    [108, 107, 106],
    [52, 51, 50],
    [80, 79, 78],
    [113, 115, 114],
    [197, 199, 198],
    [141, 143, 142],
    [169, 171, 170],
    [85, 87, 86],
    [29, 31, 30],
    [57, 59, 58],
    [116, 118, 117],
    [200, 202, 201],
    [144, 146, 145],
    [172, 174, 173],
    [88, 90, 89],
    [32, 34, 33],
    [60, 62, 61],
    [119, 121, 120],
    [203, 205, 204],
    [147, 149, 148],
    [175, 177, 176],
    [91, 93, 92],
    [35, 37, 36],
    [63, 65, 64],
    [122, 124, 123],
    [206, 208, 207],
    [150, 152, 151],
    [178, 180, 179],
    [94, 96, 95],
    [38, 40, 39],
    [66, 68, 67],
    [125, 127, 126],
    [209, 211, 210],
    [153, 155, 154],
    [181, 183, 182],
    [97, 99, 98],
    [41, 43, 42],
    [69, 71, 70],
    [128, 130, 129],
    [212, 214, 213],
    [156, 158, 157],
    [184, 186, 185],
    [100, 102, 101],
    [44, 46, 45],
    [72, 74, 73],
    [131, 133, 132],
    [215, 217, 216],
    [159, 161, 160],
    [187, 189, 188],
    [103, 105, 104],
    [47, 49, 48],
    [75, 77, 76],
    [363, 362, 361],
    [447, 446, 445],
    [391, 390, 389],
    [419, 418, 417],
    [335, 334, 333],
    [279, 278, 277],
    [307, 306, 305],
    [360, 359, 358],
    [444, 443, 442],
    [388, 387, 386],
    [416, 415, 414],
    [332, 331, 330],
    [276, 275, 274],
    [304, 303, 302],
    [337, 339, 338],
    [421, 423, 422],
    [365, 367, 366],
    [393, 395, 394],
    [309, 311, 310],
    [253, 255, 254],
    [281, 283, 282],
    [340, 342, 341],
    [424, 426, 425],
    [368, 370, 369],
    [396, 398, 397],
    [312, 314, 313],
    [256, 258, 257],
    [284, 286, 285],
    [343, 345, 344],
    [427, 429, 428],
    [371, 373, 372],
    [399, 401, 400],
    [315, 317, 316],
    [259, 261, 260],
    [287, 289, 288],
    [346, 348, 347],
    [430, 432, 431],
    [374, 376, 375],
    [402, 404, 403],
    [318, 320, 319],
    [262, 264, 263],
    [290, 292, 291],
    [349, 351, 350],
    [433, 435, 434],
    [377, 379, 378],
    [405, 407, 406],
    [321, 323, 322],
    [265, 267, 266],
    [293, 295, 294],
    [352, 354, 353],
    [436, 438, 437],
    [380, 382, 381],
    [408, 410, 409],
    [324, 326, 325],
    [268, 270, 269],
    [296, 298, 297],
];
//TODO: Can this be avoided?

enum Cmd {
    SoftReset = 0xCC,
    GlobalBrightness = 0x37,
    ComPinCtrl = 0x41,
    RowPinCtrl = 0x42,
    WriteDisplay = 0x80,
    SystemCtrl = 0x35,
    ScrollCtrl = 0x20,
}

impl From<Cmd> for u8 {
    fn from(val: Cmd) -> Self {
        val as u8
    }
}

struct Matrix {
    spi: Spi,
    offset: usize,
}

impl Matrix {
    pub fn new(spi: Spi, offset: usize) -> Result<Matrix, UnicornError> {
        let mut new_matrix = Matrix { spi, offset };

        new_matrix.write(&[Cmd::SoftReset.into()])?;
        new_matrix.write(&[Cmd::GlobalBrightness.into(), 0x01])?;
        new_matrix.write(&[Cmd::ScrollCtrl.into(), 0x00])?;
        new_matrix.write(&[Cmd::SystemCtrl.into(), 0x00])?;

        let empty_display = [0; 28 * 8];
        new_matrix.write(&[&[Cmd::WriteDisplay.into(), 0x00], &empty_display[..]].concat())?;

        new_matrix.write(&[Cmd::ComPinCtrl.into(), 0xFF])?;
        new_matrix.write(&[Cmd::RowPinCtrl.into(), 0xFF, 0xFF, 0xFF, 0xFF])?;
        new_matrix.write(&[Cmd::SystemCtrl.into(), 0x03])?;

        Ok(new_matrix)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), UnicornError> {
        self.spi.write(buffer)?;
        println!("Wrote {:?} to display", buffer);
        Ok(())
    }

    fn write_display(&mut self, buf: [u8; 28 * 8 * 2]) -> Result<(), UnicornError> {
        self.write(
            &[
                &[Cmd::WriteDisplay.into(), 0x00],
                &buf[self.offset..self.offset + (28 * 8)],
            ]
            .concat(),
        )
    }

    fn set_brightness(&mut self, brightness: f32) -> Result<(), UnicornError> {
        self.write(&[Cmd::GlobalBrightness.into(), (63_f32 * brightness) as u8])
    }

    fn shutdown(&mut self) -> Result<(), UnicornError> {
        self.write(&[Cmd::ComPinCtrl.into(), 0x00])?;
        self.write(&[Cmd::RowPinCtrl.into(), 0x00, 0x00, 0x00, 0x00])?;
        self.write(&[Cmd::SystemCtrl.into(), 0x00])?;
        Ok(())
    }
}

pub struct UnicornHATMini {
    disp: [[rgb::RGB8; ROWS]; COLS],
    left_matrix: Matrix,
    right_matrix: Matrix,
    buf: [u8; 28 * 8 * 2],
}

#[derive(Error, Debug)]
pub enum UnicornError {
    #[error(transparent)]
    SPIError(#[from] rppal::spi::Error),
}

impl UnicornHATMini {
    pub fn new(spi_max_speed_hz: u32) -> Result<UnicornHATMini, UnicornError> {
        let left_matrix = Matrix::new(
            Spi::new(Bus::Spi0, SlaveSelect::Ss0, spi_max_speed_hz, Mode::Mode0)?,
            0,
        )?;
        let right_matrix = Matrix::new(
            Spi::new(Bus::Spi0, SlaveSelect::Ss1, spi_max_speed_hz, Mode::Mode0)?,
            28 * 8,
        )?;

        let disp = [[rgb::RGB8::new(0, 0, 0); ROWS]; COLS];
        let buf = [0; 28 * 8 * 2];

        Ok(UnicornHATMini {
            disp,
            left_matrix,
            right_matrix,
            buf,
        })
    }

    /// Set Pixel at the given x/y coordinate to the given colour.
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: RGB8) {
        self.disp[x][y] = rgb;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> RGB8 {
        self.disp[x][y]
    }

    pub fn set_all(&mut self, rgb: RGB8) {
        self.disp = [[rgb; ROWS]; COLS];
    }

    pub fn clear(&mut self) {
        self.set_all(RGB8 { r: 0, g: 0, b: 0 })
    }

    pub fn set_brightness(&mut self, brightness: f32) -> Result<(), UnicornError> {
        self.left_matrix.set_brightness(brightness)?;
        self.right_matrix.set_brightness(brightness)?;
        Ok(())
    }

    pub fn show(&mut self) {
        for (index, rgb) in self.disp.iter().flat_map(|f| f.iter()).enumerate() {
            let red_address = LUT[index][0];
            let green_address = LUT[index][1];
            let blue_address = LUT[index][2];

            self.buf[red_address] = rgb.r;
            self.buf[green_address] = rgb.g;
            self.buf[blue_address] = rgb.b;
        }

        self.left_matrix.write_display(self.buf).unwrap();
        self.right_matrix.write_display(self.buf).unwrap();
    }

    pub fn shutdown(&mut self) -> Result<(), UnicornError> {
        self.left_matrix.shutdown()?;
        self.right_matrix.shutdown()?;
        Ok(())
    }
}

impl Default for UnicornHATMini {
    fn default() -> UnicornHATMini {
        UnicornHATMini::new(600000).unwrap()
    }
}

impl Drop for UnicornHATMini {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial_display() {
        let uni = UnicornHATMini::default();
        assert!(uni
            .disp
            .iter()
            .flatten()
            .all(|&f| f == RGB8 { r: 0, g: 0, b: 0 }))
    }
}
