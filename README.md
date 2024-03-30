# About
The aim of this project is to have a fully working shell (bar, desktop widgets, menus) for my own personal use.

# Progress
## Bar
- [x] Workspace support (implemented for hyprland).
- [x] Battery.
- [x] Clock.
- [ ] System tray support.
- [ ] Music player support.
- [ ] More configurable theming.
- [ ] Support for different compositors other than hyprland (more modular).

## Desktop
- [ ] Clock widget.
- [ ] Right-click context menu.

- Feel free to suggest more ideas.


# Usage
1. Clone the repository `git clone https://github.com/DerpDays/rgtkshell.git`
2. Modify (or delete) `.cargo/config.toml` since it contains build arguments specific to my system.
3. Edit `src/config.rs` to your liking.
4. Run `cargo build --release`
5. The final binary will be located at `target/release/rgtkshell`

> [!NOTE]
> If you aren't happy with the memory usage, you may try set this environment variable to force gtk4 to render using the old `cairo` backend.
> `GSK_RENDERER=cairo`
