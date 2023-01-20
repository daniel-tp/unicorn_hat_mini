use core::time;

use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;

fn main() -> Result<(), unicorn_hat_mini::UnicornError>{
    let mut uni = UnicornHATMini::default();
    uni.set_brightness(0.1)?;
    let rgb = 100;
    loop {
        uni.set_all(RGB8{r:rgb, g:rgb, b:rgb});
        uni.show();
        std::thread::sleep(time::Duration::from_millis(16));
    }
}