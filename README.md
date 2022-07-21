Launcher for rust arcade cabinet

# How to

- Put executable folders in ./apps/ (it's gitignored)
- You can copy test_apps/* to apps/ for a minimal test.

There is a binary for windows setup as an example, but sources are provided, don't forget the executable should be in `apps/(any_name)/` folder.

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

# Dev notes

- Multiple binaries per app is not really supported, as it's currently undetermined which one would be run.
- check https://github.com/fishfight/FishLauncher for ideas
