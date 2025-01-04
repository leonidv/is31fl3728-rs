#![doc = include_str!("../README.md")]

#![no_std]

use core::{fmt::Debug, ops::{Range, Shr}};

use embedded_hal::i2c::{self, I2c};

#[cfg(feature="rtt-debug")]
use rtt_target::debug_rprintln;

#[derive(Debug)]
pub enum Error<E> {
    I2C(E),
    InvalidColumnsCount,
    InvalidRowsCount,
}

/// Enumeration of all supported sizes of matrices.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MatrixDimensions {
    M8x8 = 0b00,
    M7x9 = 0b01,
    M6x10 = 0b10,
    M5x11 = 0b11,
}

/// All supported lighting intensity. 
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum LightingIntensity {
    C05mA = 0b1000,
    C10mA = 0b1001,
    C15mA = 0b1010,
    C20mA = 0b1011,
    C25mA = 0b1100,
    C30mA = 0b1101,
    C35mA = 0b1110,
    C40mA = 0b0000,
    C45mA = 0b0001,
    C50mA = 0b0010,
    C55mA = 0b0011,
    C60mA = 0b0100,
    C65mA = 0b0101,
    C70mA = 0b0110,
    C75mA = 0b0111
}

/// All supported Audio input gains
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AudioInputGain {
    G00dB = 0b0_000_0000,
    G03dB = 0b0_001_0000,
    G06dB = 0b0_010_0000,
    G09dB = 0b0_011_0000,
    G12dB = 0b0_100_0000,
    G15dB = 0b0_101_0000,
    G18dB = 0b0_110_0000,
    GMinus6dB = 0b111
}

/// Driver 
pub struct IS31FL3728<I2C> {
    i2c: I2C,
    address: u8,
    dimensions: MatrixDimensions,
    audio_input_enabled: bool,
    rows_count: u8,
    columns_count: u8,
    configuration_register: u8,
    lighting_effects_register: u8
}

const CONFIGURATION_ADDRESS: u8 = 0x00;
const UPDATE_COLUMN_ADDRESS: u8 = 0x0C;
const LIGHTING_EFFECT_ADDRESS: u8 =0x0D;
const AUDIO_EQ_ADDRESS : u8 = 0x0F;

pub const DEFAULT_LIGHTING_INTENSITY : LightingIntensity = LightingIntensity::C40mA;
pub const DEFAULT_AUDIO_INPUT_GAIN : AudioInputGain = AudioInputGain::G00dB;

const DEFAULT_CONFIGURATION_REGISTER : u8 = 0;
const DEFAULT_LIGHTING_EFFECT_REGISTER : u8 = (DEFAULT_AUDIO_INPUT_GAIN as u8) | (DEFAULT_LIGHTING_INTENSITY as u8);

impl<I2C, E> IS31FL3728<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    /// Create instance of driver
    pub fn new(
        i2c: I2C,
        address: u8,
        matrix_dimensions: MatrixDimensions,
        audio_input_enabled: bool,
    ) -> Result<IS31FL3728<I2C>, Error<E>> {
        let (rows_count, columns_count) = match matrix_dimensions {
            MatrixDimensions::M8x8 => (8, 8),
            MatrixDimensions::M7x9 => (7, 9),
            MatrixDimensions::M6x10 => (6, 10),
            MatrixDimensions::M5x11 => (5, 11),
        };

        let mut driver = IS31FL3728 {
            i2c,
            address,
            dimensions: matrix_dimensions,
            audio_input_enabled,
            rows_count,
            columns_count,
            configuration_register: DEFAULT_CONFIGURATION_REGISTER,
            lighting_effects_register: DEFAULT_LIGHTING_EFFECT_REGISTER
        };

        driver.init().unwrap();

        Ok(driver)
    }

    fn debug(&self, msg : &str, data : u8) {
        #[cfg(feature="rtt-debug")]
        debug_rprintln!("IS31FL3728: {} = {:08b}",msg, data)
    }

    fn write_config(&mut self, configuration : u8) -> Result<(),E> {
        self.i2c
                .write(self.address, &[CONFIGURATION_ADDRESS, configuration])
    }

    /// Send configuration by I2C and persist a new configuration to this instance
    fn update_lighting_effect(&mut self, configuration: u8) -> Result<(),E> {
        if self.lighting_effects_register != configuration {
            self.debug("lighting effect", configuration);
            self.i2c.write(self.address, &[LIGHTING_EFFECT_ADDRESS, configuration])?;
            self.lighting_effects_register = configuration
        }        
        Ok(())
    }

    /// Init 
    fn init(&mut self) -> Result<(), E> {
        let audio_input_mask = if self.audio_input_enabled {
            0b0_0000_1_00
        } else {
            0
        }; 

        let configuration: u8 = 0b0_0000_0_00 
            | audio_input_mask
            | self.dimensions as u8;

        if configuration != self.configuration_register {
            self.debug("configuration", configuration);
            self.write_config(configuration)?;
            self.configuration_register = configuration;
        }
        
        Ok(())
    }

    /// Update column data registers from temporary data registers.
    pub fn update(&mut self) -> Result<(), E> {
        self.i2c.write(self.address, &[UPDATE_COLUMN_ADDRESS, 0])
    }

    /// Send data to temporary registers.
    /// <div class="warning">`row_number` starts from 1.</div>
    pub fn send_column(&mut self, column_number: u8, column: u8) -> Result<(), E> {
        let msg = concat!("send column: ", stringify!(column_number));
        self.debug(msg, column);
        self.i2c
            .write(self.address, &[column_number, column])
    }

    /// Send data to temporary register and update columns registers.
    /// <div class="warning">`row_number` starts from 1.</div>
    pub fn draw_column(&mut self, column_number: u8, column: u8) -> Result<(), E> {
        self.send_column(column_number, column)?;
        self.update()
    }

    /// Send data to temporary registers and update columns registers.
    /// Picture is array of columns. 
    pub fn draw(&mut self, picture: &[u8]) -> Result<(), E> {
        for (column_idx, column) in picture.iter().enumerate() {
            let column_number = (column_idx + 1) as u8;
            self.send_column(column_number, *column)?
        }
        self.update()
    }
    

    /// Set intensity of led's matrix. 
    pub fn set_intensity(&mut self, intensity : LightingIntensity) -> Result<(),E> {
        let mask = 0b1_111_0000;
        let configuration = (self.lighting_effects_register & mask) | intensity as u8;

        self.update_lighting_effect(configuration)?;

        Ok(())
    }

    /// Set audio input gain
    pub fn set_audio_input_gain(&mut self, gain : AudioInputGain) -> Result<(),E> {
        let mask = 0b1_000_1111;
        let configuration = (self.lighting_effects_register & mask) | gain as u8;

        self.update_lighting_effect(configuration)?;

        Ok(())
    }

    /// Enable audio equalize
    pub fn audio_eq_enable(&mut self) -> Result<(),E> {
        let configuration = 0b0_1_000000;
        self.debug("Enable audio eq", configuration);
        self.i2c.write(self.address, &[AUDIO_EQ_ADDRESS,configuration])
    }

    /// Disable audio equalize
    pub fn audio_eq_disable(&mut self) -> Result<(),E> {
        let configuration = 0b0_0_000000;
        self.debug("Disable audio eq", configuration);
        self.i2c.write(self.address, &[AUDIO_EQ_ADDRESS,configuration])
    }
    
    /// Send data to temporary registers and update columns registers.
    /// Picture is array of rows. 
    /// 
    /// Use this method to simplify a work with led-matrix-editors like this one: 
    /// <https://xantorohara.github.io/led-matrix-editor/>
    pub fn draw_bitmap(&mut self, picture: &[u8;8]) ->Result<(),E> {
        let mut column_mask : u8 = 0b1000_0000;
        for column_idx in 0..=7 {
            let mut column : u8 = 0;
            for row_idx in  0..=7 {
                let pixel = picture[row_idx] & column_mask;
                let pixel_in_column = if column_idx < row_idx {
                    pixel >> (row_idx - column_idx)
                } else {
                    pixel << (column_idx - row_idx)
                };
                column = column | pixel_in_column;
            }
            self.send_column((column_idx as u8)+1, column)?;
            column_mask = column_mask >> 1;
        }

        self.update()
    }

    /// Set all led's to off. If you want just turn off matrix without 
    /// changing picture, use `software_shutdown`
    pub fn clear(&mut self) -> Result<(), E> {
        for i in 1..=self.columns_count {
            self.send_column(i, 0)?;
        }
        self.update()
    }

    /// Set all led's to on
    pub fn fill(&mut self) -> Result<(),E> {
        for i in 1..=self.columns_count {
            // if rows count less then 8, older bit will be ignored
            self.send_column(i, 0b1111_1111)?;
        }
        self.update()
    }

    /// Turn off matrix output with saving all registry.
    /// Use `software_on` to return image
    pub fn software_shutdown(&mut self) -> Result<(),E> {
        let configuration = self.configuration_register | 0b1_0000_0_00;
        self.write_config(configuration)
    }

    /// Turn on matrix output
    pub fn software_on(&mut self) -> Result<(),E> {
        let configuration = self.configuration_register & 0b0_1111_1_11;
        self.write_config(configuration)
    }
}
