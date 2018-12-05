local player

minetest.register_on_connect(function()
    player = minetest.localplayer
end)

minetest.register_globalstep(function(dtime)
    if player then
        local player_name = player:get_name()
        local player_pos = player:get_pos() or {x=0, y=0, z=0}
        local player_look = {x=0, y=0, z=0}
        local camera_pos = {x=0, y=0, z=0}
        local camera_look = {x=0, y=0, z=0}
        if minetest.camera then
            camera_pos = minetest.camera:get_pos()
            camera_pos.x = (camera_pos.x/10)
            camera_pos.y = (camera_pos.y/10)
            camera_pos.z = (camera_pos.z/10)
            camera_look = minetest.camera:get_look_dir()
            player_look = camera_look
        end
        print("mumble id "..player_name)
        print("p p ["..(player_pos.x).." "..(player_pos.y).." "..(player_pos.z).."]")
        print("p l ["..(player_look.x).." "..(player_look.y).." "..(player_look.z).."]")
        print("c p ["..(camera_pos.x).." "..(camera_pos.y).." "..(camera_pos.z).."]")
        print("c l ["..(camera_look.x).." "..(camera_look.y).." "..(camera_look.z).."]")
        print("mumble submit")
    end
end)
