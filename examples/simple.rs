use core::time;
use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;

fn main() -> Result<(), unicorn_hat_mini::UnicornError>{
    let mut uni = UnicornHATMini::default();
    uni.set_brightness(0.1)?;
    let mut rgb = 100;
    loop {
        uni.set_all(RGB8{r:rgb, g:rgb, b:rgb});
        uni.show();
        if rgb <255 {
            rgb+=1;
        }else{
            rgb=0;
        }
        std::thread::sleep(time::Duration::from_millis(16));
    }
}