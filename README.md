## Tetris but in rust

You will need to install the libSDL2-dev and libSDL2-image-dev packages for your OS. SDL2-image is used to load the .png images in the root folder.

### Keys are: 
* Arrows to move,
* A to rotate a piece counter-clockwise,
* D to rotate clockwise,
* Q to hold the current piece or swap it with a held piece.
* Space to drop the piece to the bottom.
* Esc to exit.

The game will exit when you fill the whole board and a new piece does not fit in anymore.

SRS rotation is implemented.

### Next steps:
* t-spin detection
* overhaul the scoring system
* sound and music
* more animations and player feedback


![image](https://github.com/wvandyk/tetris-rust/assets/851446/e329bc3b-8e74-4bd2-ae83-fe7136374596)
