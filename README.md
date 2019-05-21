# minetest-mumble-wrapper
A small wrapper program and client mod to enable Mumble positional audio in Minetest without having to make a custom build of Minetest.

Tutorial: https://www.youtube.com/watch?v=0rk-004yLyk

Install the csm (/minetestmod/mumble/)
Start mumble, the handler may have an error otherwise. Make sure Positional Audio and Link is enabled in Settings -> Audio Output and Settings -> Plugins. (Both need advanced options to be shown.)
Run the executable (in the releases) with the minetest program as an argument. (can be done easily with shortcuts in windows, and .sh files in linux)
It will also search in it's current directory for minetest, as well as /usr/bin/, usr/games/, C:/Program Files/minetest/bin/, and C:/Program Files (x86)/minetest/bin/ so you may not have to pass the path directly as an argument.
After a few seconds after joining a world the terminal for the handler/minetest should go a bit crazy, outputting your players location and heading. If it dosn't, the csm probably isn't correctly installed.
After that Mumble should say in it's chat "Minetest linked" If it dosn't, double check your Mumble settings and make sure Positional Audio is enabled both in Settings -> Audio Output and Settings -> Plugins.
