Launcher for rust arcade cabinet

# How to

- Put executable folders in ./apps/ (it's gitignored)
- You can copy test_apps/*.exe to apps/ for a minimal test.
- Check fullscreen example on windows and hit ESC to exit

There is binaries for windows set up as examples, but sources are provided, don't forget the executable should be in `apps/(display_name)/` folder.

Example hierarchy: 

<pre>
./apps
--- unix_app/
    --- ls.sh
--- game_using_bevy/
    --- executable_name
    --- assets/
        --- images.png
--- windows_app/
    --- test_app.exe
...
</pre>

# How to

- arrows to select game to launch
- right shift to select

# Dev notes

- Multiple binaries per app is not really supported, as it's currently undetermined which one would be run.
- check https://github.com/fishfight/FishLauncher for ideas
