use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;
use core::time;
use std::time::{SystemTime, UNIX_EPOCH};
use palette::{FromColor, Srgb, Hsv, Pixel};
fn main() -> Result<(), unicorn_hat_mini::UnicornError>{
    let mut uni = UnicornHATMini::default();
    uni.set_brightness(0.1)?;

    loop {
        let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time should not be reversed").as_millis();
        let hue:f64 = (time/10) as f64;

        let rgb: [u8; 3] = Srgb::from_color(Hsv::new(hue, 1.0, 1.0)).into_format().into_raw();
        uni.set_all(RGB8{r:rgb[0], g:rgb[1], b:rgb[2]});
        uni.show();
        std::thread::sleep(time::Duration::from_millis(10));
    }

}