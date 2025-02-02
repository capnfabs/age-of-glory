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
# draw 1440 backgrounds, takes 30 seconds-10 minutes depending on resolution, it's multithreaded but not super optimized
cargo run --release --bin draw -- --source res/bkg-2880x1864.png output-2880x1864 batch
# will chomp all your RAM and all your swap
wallpapper -i output-2880x1864/listing.json -o output_complete_2880x1864.heic
```
