use core::time;

use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;

fn main() -> Result<(), unicorn_hat_mini::UnicornError>{
    let mut uni = UnicornHATMini::default();
    let mut rgb = 100;
    loop {
        uni.set_all(RGB8{r:rgb, g:rgb, b:rgb});
        uni.show();
        rgb+=1;
        std::thread::sleep(time::Duration::from_millis(16));
    }
}