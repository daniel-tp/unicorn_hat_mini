# Unicorn_Hat_Mini Crate

[![Crates.io](https://img.shields.io/crates/v/unicorn_hat_mini.svg)](https://crates.io/crates/unicorn_hat_mini)

This is a Rust Library for interfacing with a [Pimorini Unicorn HAT Mini](https://shop.pimoroni.com/products/unicorn-hat-mini), for Raspberry Pi.

It is strongly based on their [Python Library](https://github.com/pimoroni/unicornhatmini-python) with some changes.

SPI must be enabled on your Raspberry Pi for it to work: `sudo raspi-config nonint do_spi 0`

# Example

Cargo.toml
```toml
[dependencies]
unicorn_hat_mini = "0.1"
```

Example code that sets all the pixels.
```rust
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
```
