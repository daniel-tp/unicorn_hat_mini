use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;

fn main() -> Result<(), unicorn_hat_mini::UnicornError>{
    let mut uni = UnicornHATMini::default();
    uni.set_brightness(0.1)?;
    uni.set_all(RGB8{r:255, g:255, b:255});
    uni.show();
    Ok(())
}