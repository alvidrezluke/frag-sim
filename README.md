# frag-sim

Frag-sim is a simple application designed to simulate fragmentation grenade explosions. It outputs the path of the fragments to a csv file and can be configured by editing a config.txt file in the same directory as the executable. It can also be used by passing in the "-c /path/to/config.txt" flag in order to use a config file located in a different location than default.
The executable has been shown to handle over a thousand fragments well on a RTX 3070Ti laptop GPU. Most simulation parameters are configurable and the txt file will only be read when the "Simulate" button is pressed. This allows the ability to make changes to the file between simulations. To exit a simulation press "Q". While in a simulation you can toggle your mouse controlling the moveable camera by clicking "Esc".

## About

This project was created for FSRI 2022. Feel free to clone this repository and make your own changes.

### Credits
Some parts of the main menu code are pulled from https://github.com/belzile/platformer/tree/part-9.
The physics engine is provided by the crate https://github.com/dimforge/bevy_rapier.
