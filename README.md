## Introduction
Mini project to get a feel for game development using Rust + Bevy.

The "game" is an ECS simulation of a road network with NPCs moving around on a 2 sec interval.

- https://www.rust-lang.org/
- https://bevyengine.org/

The world generator assigns 5% of the settlements to be hubs. Some settlements in each cluster
then connects to some other cluster to form one connected graph.

## Run
Clone and make sure you have Rust installed. Ie. on MAC

    $ brew install rust

Then:

    $ cargo run

## Demo
A small animated gif showing what the simulation looks like.
![Screenshot](media/animation.gif)

Screenshot showing 500 settlements and 500 NPCs.
![Screenshot](media/screenshot1.png)


## TODO
No performance optimizations or profiling made at all. Performance breaks down (on my machine) at around 10000 settlements.

CPU cores was not loaded at all so was maybe a rendering queue (no batching I believe) choke?

Would be interesting and fun to investigate and optimize to load all cores/GPU to the maximum and see how large a
simulation could be driven.
