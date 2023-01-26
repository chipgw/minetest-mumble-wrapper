local player
local serverinfo
local minetestversion = string.gsub(minetest.get_version().string, '%.', '')
minetestversion = tonumber(minetestversion)
local timer = 0

minetest.register_globalstep(function(dtime)
	if minetest.localplayer and not player then
		player = minetest.localplayer
		serverinfo = minetest.get_server_info()
		minetest.log("action", "mumble id "..player:get_name())
		minetest.log("action", "mumble context "..serverinfo.ip..":"..serverinfo.port)
		minetest.display_chat_message("!Mumble loaded! This mod uses print() to send ingame positional data, so your debug.txt may get quite large, and it's recommended to set debug_log_level to nothing if you haven't already.")
	end
    if player then
		timer = timer + dtime
		if timer > 1 then
			serverinfo = minetest.get_server_info()
			minetest.log("action", "mumble id "..player:get_name())
			minetest.log("action", "mumble context "..serverinfo.ip..":"..serverinfo.port)
			timer = 0
		end
        local player_pos = player:get_pos() or {x=0, y=0, z=0}
        local player_look = {x=0, y=0, z=0}
        local camera_pos = {x=0, y=0, z=0}
        local camera_look = {x=0, y=0, z=0}
        if minetest.camera then
            camera_pos = minetest.camera:get_pos()
			--before Minetest 5.2.0 camera positions were 10x player position.
			if minetestversion and minetestversion <= 520 then
				camera_pos.x = (camera_pos.x/10)
				camera_pos.y = (camera_pos.y/10)
				camera_pos.z = (camera_pos.z/10)
			end
            camera_look = minetest.camera:get_look_dir()
            player_look = camera_look
        end
        minetest.log("action", "p p ["..(player_pos.x).." "..(player_pos.y).." "..(player_pos.z).."]")
        minetest.log("action", "p l ["..(player_look.x).." "..(player_look.y).." "..(player_look.z).."]")
        minetest.log("action", "c p ["..(camera_pos.x).." "..(camera_pos.y).." "..(camera_pos.z).."]")
        minetest.log("action", "c l ["..(camera_look.x).." "..(camera_look.y).." "..(camera_look.z).."]")
        minetest.log("action", "mumble submit")
    end
end)
