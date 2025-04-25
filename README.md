# Pallet Finder

An utility binary to extract dominant colors from images. You can use it to extract color pallet from images.

## Installation
First you need to setup dependencies specified in [ImageMagick](https://github.com/nlfiedler/magick-rust).
I did not put this on crates.io so you can't install directly from there. Instead you can clone the repo and then:
```bash
cargo install --path .
```
in the directory.

## Usage
The simple usage gives you the color pallet hex codes in order of dominance in the image.
```bash
pallet_finder <path to input image>

#2E3441
#373E4D
#3D4454
#484C5E
#4D586E
#50607A
#835C68
#B47071
#536683
#556B8B
#577193
#7A7C94
#A293A5
#CCA9A1
#DECCB5
#B6B0C2
```

To get colorful output, use the flag --print_colors and to get the pallet as SVG you can use --output <path to save the svg> to save it.

## Notes
I will try to remove the dependency on Magick and publish the package to crates.io for ease of installation.
