use rgb::RGB8;
use unicorn_hat_mini::UnicornHATMini;

pub fn main() {

    let mut uni = UnicornHATMini::default();

    uni.set_all(RGB8 {r:255, g:0, b:255});
    loop {
        uni.show();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

}