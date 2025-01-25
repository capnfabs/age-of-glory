## Plan

- [scope/done] Start by generating wallpaper for the clock. Not perfect but good enough
- [scope] Then make a dynamic wallpaper file manually for... 1440 screenshots and assess the damage
- [build] Then make the wallpaper perfect
- [scope / decided against] Then consider building the website to make it possible to generate and download for all resolutions... but actually I can probably just run it for a bunch of screen resolutions? Don't need to host infra anywhere.
  - I think this is going to be impossible because the code which compresses the HEIF is too slow (takes about ~0.5s per image in release mode on my mac, can't multithread it easily)
- [??] A windows XP one would be sickkk too


## Final product ideas:

### cmdline
A command line program called `taskbar`, something like this

(I don't know how I feel about this yet)

```
taskbar --help

Puts a Windows Taskbar on a background.

--resolution: (required)
--mode: which aesthetic to use, supports '98' and 'XP'
--icons: whether to add the holy trinity of icons (My Computer / Recycle Bin)
--animated: animates the clock.

```

### website
Something that automatically detects your resolution and basically lets you "download a taskbar" (this is very funny, it's even very funnier if the clock works... can probably target a bunch of screen sizes)

## Notes

> Microsoft Sans Serif font is a very legible User Interface (UI) font. It was designed to be metrically compatible with the MS Sans bitmap font that shipped in early versions of Microsoft Windows.

from https://learn.microsoft.com/de-de/typography/font-list/microsoft-sans-serif

