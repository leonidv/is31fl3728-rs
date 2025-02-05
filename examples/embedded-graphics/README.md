# Example of usage IS31FL3728-rs with embedded-graphics

## Quick tutorial

1. Install [Aseprite](https://www.aseprite.org/) — simple, but powerful, open-source pixel art tool
2. Create new Sprite:
   - Size: 8x8px, 
   - Color mode: Indexed
   - Background: Black
3. Zoom it and draw first sprite.
4. In the bottom panel, RMB on 1 and select “New Frame”.
5. Aseprite creates copy of the first frame. Modify it.
6. Repeat step 5 and create animation. You can make a preview by clicking the Play button above the list  of sprites.
7. File export and select “PNG” extensions. Aseprite confirms that each frame will be exported as an  independent file.
8. Use ImageMagick to convert PNG into BMP ([png2bpm.sh](media/png2bmp.sh)):
   ```bash
   #!/usr/bin/env bash

    for f in *.png
        do   magick $f -monochrome "${f%.*}".bmp
    done
   ``` 
9. See example how to play the animation on the LED Matrix 8x8 🥳

## Hardware

Hardware: Nucleo-64 [STM32F041RE](https://www.st.com/en/evaluation-tools/nucleo-f401re.html).
HAL:  [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal). 

Connecting via I2C1:
* SCL - PB8
* SDA - PB9.