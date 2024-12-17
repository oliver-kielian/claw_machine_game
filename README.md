#Claw machine Game Bevy 0.14.2

NOTE:
Bevy did have an update(0.15) as I was creating this. I made the decsion for this project I would stick with 0.14.2 as I am still new and don't want to risk loosing progress to try an change my current code to reflect this update.
That being said, I will either return or launch around project to explore this new update.

Final for my CS 310 class, Programming Projects in X (Rust). One option for a final project was to create a simple 2D game. I want to go into game development so I picked that. Bevy was the choice for the engine given its popularity
and the likely uprise of using Rust in game development. This game is a simple Claw Machine game who's puprose is both a passing grade and to learn the mechanics of bevy. The main features include the machine, the claw, the balls, and the cats. 
The game logic is also included.



HOW TO PLAY:
Upon running the app, you will be greeted with low bit arcade music and a claw machine. To the left there will be simple directions describing how to play. I will put them more in depth here.

To move the claw to the left, hold and press the left arrow key on your keyboard.
To move the claw to the right, hold and press the right arrow key on your keyboard.

To drop the claw press space.

To raise the claw press space once again.

The claw will now be holding onto a ball. You have anywhere from 1 second to 5 seconds to drop it into the return area.

Failure to drop the ball into the return area will result in nothing happening. Success in dropping the ball into the return area will result in a box with they cat you have won in it.

Press enter to dismiss the box and to keep playing.




USE:
People can use this code to play a simple game, build off to create a more complex claw machine game, or to attempt to better understand bevy themselves. Since a new version was released before I could finish this project
I recommned looking at https://bevyengine.org/learn/migration-guides/0-14-to-0-15/ to migrate this code to 0.15 if you are looking at it before I get a chance to.



Sources:

Art:
    Cats:
    ToffeeBunny on itch.io
    https://toffeebunny.itch.io/pirate-cats

    Machine:
    Saccharineheartx on tumblr

Sound:
    Background: 
    https://pixabay.com/music/video-games-pixel-fight-8-bit-arcade-music-background-music-for-video-208775/

    Cat Meow:
    https://pixabay.com/users/freesound_community-46691455/



Code:
    https://stackoverflow.com/questions/66199782/how-to-add-a-background-image-to-2d-game-in-bevy
    https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/
    https://bevy-cheatbook.github.io/input/keyboard.html
    https://www.youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd
    https://taintedcoders.com/bevy/audio
    https://docs.rs/crate/bevy_sprite/0.15.0/source/src/bundle.rs
    https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust
    https://bevyengine.org/examples/2d-rendering/sprite-sheet/
    https://www.youtube.com/watch?v=iW19V3a96tYc
    https://docs.rs/bevy/0.14.2/bevy/prelude/struct.TextBundle.html#structfield.text
    https://www.youtube.com/watch?v=gy2G63SA-W8&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=7
    https://westernstatecoloradou-my.sharepoint.com/:w:/g/personal/oliver_kielian_western_edu/EaoaWiRo8N5Io_DrRof8_KAB2U20i7IZLc8KCyqD2qBQGw?e=DbxDsw

    
    License:

    This project has a simple MIT License
