## Status

IT WORKS

## Plan

- [scope/done] Start by generating wallpaper for the clock. Not perfect but good enough
- [scope/done] Then make a dynamic wallpaper file manually for... 1440 screenshots and assess the damage
  - Do this with wallpapper
- [build] Then make the wallpaper perfect
- [build] Run it for a bunch of screen resolutions? Don't need to host infra anywhere.
- [??] A windows XP one would be sickkk too


## Final product ideas:

### cmdline
A command line program called `taskbar`, something like this

(I think this is pretty cute actually)

```
taskbar --help

Puts a Windows Taskbar on a background.

--resolution: (required)
--mode: which aesthetic to use, supports '98' and 'XP'
--apps: notepad, solitaire etc
--icons: whether to add the holy trinity of icons (My Computer / Recycle Bin)
--clock-time: time on the clock

```

### website
Something that automatically detects your resolution and basically lets you "download a taskbar" (this is very funny, it's even very funnier if the clock works... can probably target a bunch of screen sizes)

## Notes

> Microsoft Sans Serif font is a very legible User Interface (UI) font. It was designed to be metrically compatible with the MS Sans bitmap font that shipped in early versions of Microsoft Windows.

from https://learn.microsoft.com/de-de/typography/font-list/microsoft-sans-serif

## Blog post

Are you a macOS user? Do you enjoy the incredible battery life, and the elegant user experience (link to upside down mouse)? And yet, do you ever stare at your empty desktop in the morning, and think... wow, I wish I just had a little bit more inspiration? A little bit more motivation? A bit more enthusiasm?

Thinking, "I wish i just knew where to start"?

Introducing: [download a taskbar] for OSX.

- Figure out most common resolutions
- Precompute a bunch of options
- Add JS for detecting screen dimensions

# Steps:
1. Download a taskbar
2. Walk through settings
3. blah blah blah
4. You can also customize your experience by downloading the `taskbar` app


## File formats
- This is an HEIC file, containing an XMP file, containing an RDP file, containing a base64-encoded plist.
- XMP is amazing because the [spec](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf) has this cursed bit (page 10/11) so that software that doesn't know about XMP can still (a) detect and read the XMP (b) sometimes write the XMP??? Incredible stuff.


## Here's how I did it:
- Spent two days tearing out my hair because my custom Rust code to assemble the image didn't work
- I could produce an image that _seemed_ appropriate when I compared it to dynamic wallpapers from other solutions, but it just didn't work, OSX logs didn't given any info, thumbnail didn't show on wallpapers tab
- Then eventually just googled for command line apps which can do it
- (I would like to remind y'all that I'm very good at my job and still have this failure mode)
