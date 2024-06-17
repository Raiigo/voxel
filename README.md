# Presentation

The goal of this project is to implement a voxel engine in Rust, using [Bevy](https://bevyengine.org/) engine, it is focused on performance which means :  

- Fast chunk mesh generation
- Optimal chunk mesh generation
- Fast chunk loading from disk
- Fast chunk generation
- Low disk usage (high compression ratio)
- Fast rendering (using good meshing algorithm)

# TODO

Main things I have to work on

| Not done                   | WIP                                | Done               |
| -------------------------- | ---------------------------------- | ------------------ |
| Adding good chunk loading  |                                    |                    |
|                            |                                    | Adding fast mesher |
| Adding performant mesher   |                                    |                    |
| Refactor all engine        |                                    |                    |
|                            | Adding chunk compressing algorithm |                    |
| Implementing asset loading |                                    |                    |