#![deny(unsafe_code)]
#![no_main]
#![no_std]


use is31fl3728_rs::{LightingIntensity, MatrixDimensions, DEFAULT_LIGHTING_INTENSITY, IS31FL3728};

// Halt on panic
use panic_rtt_target as _;

use rtt_target::{debug_rprintln, rtt_init_default, rtt_init_print};

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

    led_matrix.clear().unwrap();

    let picture = [
        0b00000000,
        0b01100110,
        0b11111111,
        0b11111111,
        0b11111111,
        0b01111110,
        0b00111100,
        0b00011000,
    ];

    led_matrix.draw_bitmap(&picture).unwrap();
    led_matrix.set_intensity(LightingIntensity::C20mA).unwrap();

    loop {
            led_matrix.software_on().unwrap();
            led_matrix.set_intensity(LightingIntensity::C10mA).unwrap();
            delay.delay_ms(150);
            led_matrix.set_intensity(LightingIntensity::C25mA).unwrap();
            delay.delay_ms(150);
            led_matrix.set_intensity(DEFAULT_LIGHTING_INTENSITY).unwrap();
            delay.delay_ms(500);

            led_matrix.set_intensity(LightingIntensity::C25mA).unwrap();
            delay.delay_ms(150);
            led_matrix.set_intensity(LightingIntensity::C10mA).unwrap();
            delay.delay_ms(150);
            led_matrix.software_shutdown().unwrap();
            delay.delay_ms(300);        
    }
}

