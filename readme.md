# pmd3D script runner (tempory name)

This is a tool I write, with the goal being to execute cutscene from the two mystery dungeon that got released on the 3DS (pokemon mystery dungeon: gate to infinity and pokemon super mystery dungeon).

It is actually of little use in the current state, but can run some minimal exemple. In it's current state, it use 2D graphic from pokemon mystery dungeon: explorer of sky.

## How to use it
To run the program and compile everything, you first need to have rust/cargo installed. Then run: "cargo run" and install dependencies that are required when you have an error (depend on the renderer, which depend on the OS, I think, but is otherwise light on dependancies).
You also need to have a folder named data/ that contain an extracted explorer of sky rom (to help you, it just need the file data/MONSTER/m_ground.bin)

## Done:
lua runner ( a bit ugly, but work )

logic: some lua api, character can move, screen can fade, portrait can appear

render: character appear/move with animation based on sprites of explorer of sky. screen can face, but no portrait appear.

## to do:

skip to 3d (keep the 2D renderer as an option, can allow a lot of cool stuff)

read plb file (at the json output of the eddyk28 tool)
