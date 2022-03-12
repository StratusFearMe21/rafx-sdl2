# Rafx SDL2

This is a fork of [rafx](https://github.com/aclysma/rafx) in which the Vulkan backend creates surfaces using `sdl2` instead of `raw-window-handle`. This allows for the full suite of video backends that SDL2 supports to be usable without crashing.

For example, `sdl2` is capable of opening windows on Linux without the need for Wayland or X. However, `raw-window-handle` *requires* one of them and does not support [KMS/DRM](https://en.wikipedia.org/wiki/Direct_Rendering_Manager). Thus the need for this fork.

This is a companion crate for [grezi](https://github.com/StratusFearMe21/grezi)
