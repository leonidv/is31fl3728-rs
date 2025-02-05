#!/usr/bin/env bash

for f in *.png
    do   magick $f -monochrome "${f%.*}".bmp
done
