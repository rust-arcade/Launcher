Launcher for rust arcade cabinet

# How to

Put executable folders in ./apps/ (it's gitignored)

You can copy test_apps to apps for a minimal test.

There is a binary for windows setup as an example, but sources are provided, don't forget the executable should be in `apps/(any_name)/` folder.

example hierarchy: 

<pre>
./apps
--- ls/
    --- ls.sh
--- bevy_app/
    --- executable
    --- assets/
        --- images.png
...
</pre>

# Dev notes

- check https://github.com/fishfight/FishLauncher for ideas
