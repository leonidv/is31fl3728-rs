#![deny(unsafe_code)]
#![no_main]
#![no_std]


use is31fl3728_rs::{LightingIntensity, MatrixDimensions, IS31FL3728};
use tinybmp::Bmp;
use embedded_graphics::{image::Image, pixelcolor::BinaryColor, prelude::*};


// Halt on panic
use panic_rtt_target as _;

use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, gpio::GpioExt, i2c::I2c, pac, prelude::*};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    rtt_init_print!();    
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(84.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);      


    let gpiob = dp.GPIOB.split();

    let pb8_scl = gpiob.pb8;
    let pb9_sda = gpiob.pb9;

    let i2c1 = I2c::new(
        dp.I2C1,
        (pb8_scl, pb9_sda),
        hal::i2c::Mode::standard(400.kHz()),
        &clocks,
    );

    let matrix_addr: u8 = 0x60;
    let mut led_matrix = IS31FL3728::new(i2c1, matrix_addr, MatrixDimensions::M8x8, false).unwrap();


    let sprite = include_bytes!("../media/flash1.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image1 = Image::new(&bmp, Point::new(0, 0));

    let sprite = include_bytes!("../media/flash2.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image2 = Image::new(&bmp, Point::new(0, 0));

    let sprite = include_bytes!("../media/flash3.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image3 = Image::new(&bmp, Point::new(0, 0));

    let sprite = include_bytes!("../media/flash4.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image4 = Image::new(&bmp, Point::new(0, 0));

    let sprite = include_bytes!("../media/flash5.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image5 = Image::new(&bmp, Point::new(0, 0));

    let sprite = include_bytes!("../media/flash6.bmp");
    let bmp :Bmp<BinaryColor> = Bmp::from_slice(sprite).unwrap();
    let image6 = Image::new(&bmp, Point::new(0, 0));

    let sprites = [image1, image2, image3, image4, image5, image6];
    
    loop {
        let mut lightness = LightingIntensity::C10mA;

        for sprite in sprites {
            led_matrix.set_intensity(lightness).unwrap();
            sprite.draw(&mut led_matrix).unwrap();
            delay.delay_ms(250);
            lightness = lightness.next().next();            
        }            
    }
}