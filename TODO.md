## Status

I have the code embedding the images 'working', and it includes the metadata, but they're not being interpreted by the OS correctly. It's hard to know what the problem is. Ideas:
- maybe I can read the OS logs to figure it out? (nope)
- behavior is: it sets the desktop to the primary image, and then crashes.
- maybe I can compare the output of my code against the HEIC reference?
- maybe I can find some other program to create HEICs?
- maybe i can fork that preexisting software and do it in Xcode?
- there's a comment on this video saying that "time" mode doesn't work?
  - https://www.youtube.com/watch?v=irHQrbzpUyc
  - https://github.com/mczachurski/wallpapper
  - ahhh but actually, this works in Equinox? I should probably check minute resolution though.

So next steps are:
- Derisk using Equinox (try setting 12, 1, 1:01, 1:02, 1:03, 1:04...)
  - WORKS
- Use Wallpapper
  - WORKS but I have to fuck around with the timezones
  - Ok I shouldn't have fucked with the timezones this much.

IT WORKS

## Plan

- [scope/done] Start by generating wallpaper for the clock. Not perfect but good enough
- [scope] Then make a dynamic wallpaper file manually for... 1440 screenshots and assess the damage
  - Do this with a hacked up version of Equinox.
- [build] Then make the wallpaper perfect
- [scope / decided against] Then consider building the website to make it possible to generate and download for all resolutions... but actually I can probably just run it for a bunch of screen resolutions? Don't need to host infra anywhere.
  - I think this is going to be impossible because the code which compresses the HEIF is too slow (takes about ~0.5s per image in release mode on my mac, can't multithread it easily)
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
