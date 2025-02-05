use core::fmt::Debug;

use embedded_graphics_core::prelude::*;
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor, prelude::OriginDimensions};

use crate::{DriverError, IS31FL3728};

impl<E> OriginDimensions for IS31FL3728<E> {
    fn size(&self) -> Size {
        return Size::new(self.columns_count.into(), self.rows_count.into());
    }
}

impl<I2C, E> DrawTarget for IS31FL3728<I2C> 
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: Debug
{
    type Color = pixelcolor::BinaryColor;

    type Error = DriverError<E>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), DriverError<E>>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        let max_x: i32 = (self.columns_count - 1).into();
        let max_y: i32 = (self.rows_count - 1).into();

        let mut buffer: [u8; crate::MAX_COLUMNS] = [0; crate::MAX_COLUMNS];

        for Pixel(coord, color) in pixels.into_iter() {
            // the buffer filled by 0 (each pixel is "off"), so process only "on" pixels.
            if color.is_on() {
                let (x, y) = (coord.x, coord.y);
                if (x >= 0 && x <= max_x) && (y >= 0 && y <= max_y) {
                    let column_idx = x as usize;
                    buffer[column_idx] = buffer[column_idx] | (0b1000_0000 >> y);
                };
            }
        }

        for idx in 0..self.columns_count {
            self.send_column(idx+1, buffer[idx as usize])?
        }

        self.update()
    }
}
