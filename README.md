# age of glory

The Windows 98 Start Menu. Now for macOS.

## Dependencies

```sh
brew tap mczachurski/wallpapper
brew install wallpapper
```

(and rust)

## Build the wallpaper

```sh
cd [this directory]
# draw 1440 backgrounds, takes about 30 seconds (multithreaded)
cargo run --release --bin draw
# takes <1 sec, make JSON listing
cargo run --release --bin makelisting
# will chomp all your RAM and all your swap
wallpapper -i output/listing.json -o output_wallpapper.heic
```
