# minetest-mumble-wrapper
A small wrapper program and client mod to enable Mumble positional audio in Minetest without having to make a custom build of Minetest.

Tutorial: https://www.youtube.com/watch?v=0rk-004yLyk

For Windows Users: There is a prepackaged zip of minetest that has been configured for use with mumble in Releases, this the easiest way to get set, if you don't mind a fresh install. Also see minetest-mumble-helpers for installing on an existing install if minetest.

Installation Directions:
Install the Client Side Mod (CSM) (/minetestmod/mumble/) or minetest-mumble-CSM.zip in releases.
For more information of how to install Client Side Mods: https://wiki.minetest.net/Installing_Client-Side_Mods

Start mumble, the handler may have an error otherwise. Make sure Positional Audio and Link is enabled in Settings -> Audio Output and Settings -> Plugins. (Both need advanced options to be shown.)
Run the executable (in the releases) with the minetest program as an argument. (can be done easily with shortcuts in windows, and .sh files in linux)
It will also search in it's current directory for minetest, as well as /usr/bin/, usr/games/, C:/Program Files/minetest/bin/, and C:/Program Files (x86)/minetest/bin/ so you may not have to pass the path directly as an argument.
After a few seconds after joining a world the terminal for the handler/minetest should go a bit crazy, outputting your players location and heading. If it dosn't, the csm probably isn't correctly installed, see https://wiki.minetest.net/Installing_Client-Side_Mods
After that Mumble should say in it's chat "Minetest linked" If it dosn't, double check your Mumble settings and make sure Positional Audio is enabled both in Settings -> Audio Output and Settings -> Plugins.

You may want to change the Positional Audio settings in Mumble in Audio Output (needs advanced settings to be visible.)
The main things you'll be changing is Maximum Distance and Minimum Volume,
Maximum Distance: How far before the volume is set to the Minimum Volume.
Minimum Volume: no matter how far away you are from someone else, you will hear them at least this loud. Set to 0 for Proximity chat.

minetest-mumble-helpers:
Note: The helpers are batch files and Windows only.
This zip file contains two batch files called "autoinstaller.bat"and "autolauncher.bat"
To use the autoinstaller:
	1: Download and unzip the minetest-mumble-helpers.zip file into your minetest directory: where the "bin", "clientmods", and "mods" folders are.)
	2: Unzip the file in place, not creating a new directory. (for 7zip use "Extract Here") Should be something like "Extract to C:\Path\To\minetest-5.0.1-win64\" NOT "Extract to C:\Path\To\minetest-5.0.1-win64\minetest-mumble-helpers\"
	3: Ensure the files were extracted correctly: all the contents should be visible from the main minetest folder. Also close any Minetest sessions running, as they may interfere with the autoinstaller's config writing.
	4: Run autoinstaller.bat, Windows may ask you if you want to run it, (perhaps even say that it could be malware) so let it run. (if you wish to see what you are doing by running the autoinstaller, rightclick it and select "edit")
	The autoinstaller should have moved the files to the correct positions and updated your Minetest settings. This should take care of all minetest related setup for mumble PA if sucessful
	If it was unsucessful, install manually.
To use the autolauncher:
	1: If it's not there already, place the autolauncher.bat into the bin folder, along with the wrapper program.
	2: Rightclick to edit the file.
	3: Ensure the paths to the Mumble executable and the name of the wrapper program is correct. Note: the /min in front of the mumble path makes mumble start minimized, if you don't like this, you can delete it.
	Running the autolauncher should start mumble and have it autojoin the correct channel, start minetest, and also close mumble once minetest is closed.
	I would recommend making a shortcut to the autolauncher and setting the icon/name if you use it often. 	
