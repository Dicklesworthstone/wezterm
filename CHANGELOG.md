# Changelog

All notable changes to [WezTerm](https://github.com/wezterm/wezterm) are documented here.
This file is maintained in the [Dicklesworthstone/wezterm](https://github.com/Dicklesworthstone/wezterm) fork.

WezTerm is a GPU-accelerated cross-platform terminal emulator and multiplexer written in Rust.
Releases are named using the pattern `YYYYMMDD-HHMMSS-commithash`.
The upstream project also publishes crate-level tags (e.g., `termwiz-0.23.3`) for its library components.

> **Agent note**: This changelog was reconstructed from `git log`, `git tag`, `gh release list`,
> and the project's `docs/changelog.md`. All commit links point to the fork repository.
> For the upstream project's own detailed per-release notes, see <https://wezterm.org/changelog.html>.

---

## Table of Contents

- [Unreleased (Nightly)](#unreleased-nightly) -- 805+ commits since last stable
- [20240203-110809-5046fc22](#20240203-110809-5046fc22) -- 2024-02-03 (Latest Stable)
- [20240128-202157-1e552d76](#20240128-202157-1e552d76) -- 2024-01-28
- [20240127-113634-bbcac864](#20240127-113634-bbcac864) -- 2024-01-27
- [20230712-072601-f4abf8fd](#20230712-072601-f4abf8fd) -- 2023-07-12
- [20230408-112425-69ae8472](#20230408-112425-69ae8472) -- 2023-04-08
- [20230326-111934-3666303c](#20230326-111934-3666303c) -- 2023-03-26
- [20230320-124340-559cb7b0](#20230320-124340-559cb7b0) -- 2023-03-20
- [20221119-145034-49b9839f](#20221119-145034-49b9839f) -- 2022-11-19
- [20220905-102802-7d4b8249](#20220905-102802-7d4b8249) -- 2022-09-05
- [20220807-113146-c2fee766](#20220807-113146-c2fee766) -- 2022-08-07
- [20220624-141144-bd1b7c5d](#20220624-141144-bd1b7c5d) -- 2022-06-24
- [20220408-101518-b908e2dd](#20220408-101518-b908e2dd) -- 2022-04-08
- [20220319-142410-0fcdea07](#20220319-142410-0fcdea07) -- 2022-03-19
- [20220101-133340-7edc5b5a](#20220101-133340-7edc5b5a) -- 2022-01-01
- [20211205-192649-672c1cc1](#20211205-192649-672c1cc1) -- 2021-12-05
- [20211204-082213-a66c61ee9](#20211204-082213-a66c61ee9) -- 2021-12-04
- [20210814-124438-54e29167](#20210814-124438-54e29167) -- 2021-08-14
- [20210502-154244-3f7122cb](#20210502-154244-3f7122cb) -- 2021-05-02
- [20210405-110924-a5bb5be8](#20210405-110924-a5bb5be8) -- 2021-04-05
- [20210314-114017-04b7cedd](#20210314-114017-04b7cedd) -- 2021-03-14
- [20210203-095643-70a364eb](#20210203-095643-70a364eb) -- 2021-02-03
- [20201101-103216-403d002d](#20201101-103216-403d002d) -- 2020-11-01
- [20200909-002054-4c9af461](#20200909-002054-4c9af461) -- 2020-09-09
- [20200718-095447-d2315640](#20200718-095447-d2315640) -- 2020-07-18
- [20200620-160318-e00b076c](#20200620-160318-e00b076c) -- 2020-06-20
- [20200517-122836-92c201c6](#20200517-122836-92c201c6) -- 2020-05-17
- [20200503-171512-b13ef15f](#20200503-171512-b13ef15f) -- 2020-05-03
- [20200406-151651-5b700e4](#20200406-151651-5b700e4) -- 2020-04-06
- [20200202-181957-765184e5](#20200202-181957-765184e5) -- 2020-02-02
- [20200113-214446-bb6251f](#20200113-214446-bb6251f) -- 2020-01-13
- [20191218-101156-bf35707](#20191218-101156-bf35707) -- 2019-12-18
- [20191124-233250-cb9fd7d](#20191124-233250-cb9fd7d) -- 2019-11-24
- [Earlier Releases (2019)](#earlier-releases-2019)
- [Crate-Level Tags](#crate-level-tags)

---

## Unreleased (Nightly)

> 805+ commits on `main` since `20240203-110809-5046fc22`.
> The Wayland backend has been substantially rewritten, new tmux control mode support added,
> extensive crate refactoring of termwiz, and a large number of new configuration options landed.

### Architecture / Refactoring

- **termwiz crate split**: Extracted `wezterm-cell`, `wezterm-char-props`, `wezterm-surface`, and `wezterm-escape-parser` as independent crates from the monolithic `termwiz` library. The escape parser can now build in `no_std` environments. ([304fb40](https://github.com/Dicklesworthstone/wezterm/commit/304fb4056), [6740628](https://github.com/Dicklesworthstone/wezterm/commit/6740628da), [80de3aa](https://github.com/Dicklesworthstone/wezterm/commit/80de3aa31), [2b15e46](https://github.com/Dicklesworthstone/wezterm/commit/2b15e4642))
- termwiz: Made tmux control code and image support optional via Cargo features. ([33b40fe](https://github.com/Dicklesworthstone/wezterm/commit/33b40fe62), [b4261af](https://github.com/Dicklesworthstone/wezterm/commit/b4261af93))
- vtparse: Added `alloc` feature for `no_std` environments. ([c024130](https://github.com/Dicklesworthstone/wezterm/commit/c02413027))
- RPM packaging split into `wezterm-common`, `wezterm-gui`, and `wezterm-mux-server` subpackages. ([b647d74](https://github.com/Dicklesworthstone/wezterm/commit/b647d74fd))
- macOS: Migrated to `objc2` bindings. ([d3d9d8e](https://github.com/Dicklesworthstone/wezterm/commit/d3d9d8e39))
- Dependencies: wgpu 24 to 25, harfbuzz to 11.2.1, libssh to 0.11.1, freetype to 2.13.3. ([b249d9c](https://github.com/Dicklesworthstone/wezterm/commit/b249d9c42), [f412c81](https://github.com/Dicklesworthstone/wezterm/commit/f412c817c))

### Wayland Rewrite

- **Major rewrite of the Wayland backend** using smithay-client-toolkit 0.18/0.19, contributed by @tzx, @tmccombs, @deviant, and many others. This includes proper window frame support, IME support, drag-and-drop, primary selection, tiled/maximized state handling, and better compositor compatibility. ([7c77f40](https://github.com/Dicklesworthstone/wezterm/commit/7c77f407f), [ccb7270](https://github.com/Dicklesworthstone/wezterm/commit/ccb7270dd), [a7ff718](https://github.com/Dicklesworthstone/wezterm/commit/a7ff718c7))

### tmux Control Mode (`tmux -CC`)

- **tmux -CC control mode** is now very usable: pane sync, tab/window management, split pane support, layout sync, performance improvements, and backward compatibility for older tmux versions. Thanks to @joexue. ([718d91a](https://github.com/Dicklesworthstone/wezterm/commit/718d91a62) through [7e0e15a](https://github.com/Dicklesworthstone/wezterm/commit/7e0e15a13))
- Fixed tmux -CC mode not parsing output correctly. ([8fb60ad](https://github.com/Dicklesworthstone/wezterm/commit/8fb60ad82))
- Fixed tmux -CC error on `%config-error`. ([4ce83f0](https://github.com/Dicklesworthstone/wezterm/commit/4ce83f093))
- Fixed stack overflow on Windows when using tmux -CC. ([574e531](https://github.com/Dicklesworthstone/wezterm/commit/574e53190))

### New Features

- **ConEmu progress reporting**: Preliminary support for ConEmu-style progress escape sequences; see `pane:get_progress()`. ([44866cc](https://github.com/Dicklesworthstone/wezterm/commit/44866cc13), [fb023956](https://github.com/Dicklesworthstone/wezterm/commit/fb023956c))
- **`wezterm.serde` module**: JSON/TOML/YAML serialization and deserialization from Lua. Thanks to @expnn. ([8596422](https://github.com/Dicklesworthstone/wezterm/commit/8596422ab))
- **SSH agent forwarding**: `wezterm ssh` and SSH multiplexer domains now support agent forwarding with automatic `SSH_AUTH_SOCK` maintenance. Thanks to @Riatre. ([9b811c7](https://github.com/Dicklesworthstone/wezterm/commit/9b811c7a1), [4af418f](https://github.com/Dicklesworthstone/wezterm/commit/4af418fdd))
- **SSH `ServerAliveInterval`**: libssh backend now respects `ServerAliveInterval`. ([909573f](https://github.com/Dicklesworthstone/wezterm/commit/909573fad))
- **SSH `ProxyUseFdPass`**: wezterm-ssh now supports `ProxyUseFDpass`. Thanks to @loops. ([caf450b](https://github.com/Dicklesworthstone/wezterm/commit/caf450b87))
- **`window_content_alignment`** option to control where excess pixel gap is placed. Thanks to @Shiphan. ([94a4252](https://github.com/Dicklesworthstone/wezterm/commit/94a4252f2))
- **`Confirmation`** key assignment for prompts before destructive actions. Thanks to @mgpinf. ([2dda592](https://github.com/Dicklesworthstone/wezterm/commit/2dda592ba))
- **`ShowLauncherArgs`** customizable help text and alphabet option. Thanks to @mgpinf. ([4c32d6f](https://github.com/Dicklesworthstone/wezterm/commit/4c32d6fd5), [46f74a5](https://github.com/Dicklesworthstone/wezterm/commit/46f74a5bf))
- **`MACOS_FORCE_SQUARE_CORNERS`** option for `window_decorations`. Thanks to @amadeusdotpng. ([fcae187](https://github.com/Dicklesworthstone/wezterm/commit/fcae187b2))
- **macOS titlebar background matching**: `MACOS_USE_BACKGROUND_COLOR_AS_TITLEBAR_COLOR` option. Thanks to @Jay-Madden. ([c0b29fe](https://github.com/Dicklesworthstone/wezterm/commit/c0b29fef2))
- **`macos_fullscreen_extend_behind_notch`** option. Thanks to @wryanzimmerman. ([c6b6787](https://github.com/Dicklesworthstone/wezterm/commit/c6b67877b))
- **`QuickSelectArgs`** `skip_action_on_paste` option. Thanks to @nhurlock. ([b374d38](https://github.com/Dicklesworthstone/wezterm/commit/b374d383c))
- **`quick_select_remove_styling`** option. Thanks to @mgpinf. ([85409e5](https://github.com/Dicklesworthstone/wezterm/commit/85409e57a))
- **Unicode 16 octant characters** when `custom_block_glyphs` is enabled. Thanks to @eschnett. ([94610314](https://github.com/Dicklesworthstone/wezterm/commit/94610314e))
- **Git branch, progress bar, and spinner glyphs** added to `custom_block_glyphs`. Thanks to @BenBergman. ([275833e](https://github.com/Dicklesworthstone/wezterm/commit/275833e21), [2edbb81](https://github.com/Dicklesworthstone/wezterm/commit/2edbb81d2), [18049fa](https://github.com/Dicklesworthstone/wezterm/commit/18049faf6))
- **`cell_widths`** option for explicit cell width control. Thanks to @hamano. ([8ec1f24](https://github.com/Dicklesworthstone/wezterm/commit/8ec1f2451))
- **`text_min_contrast_ratio`** and **`reverse_video_cursor_min_contrast`** options. Thanks to @jameshurst. ([4a210168](https://github.com/Dicklesworthstone/wezterm/commit/4a210168e), [af8d521](https://github.com/Dicklesworthstone/wezterm/commit/af8d521ef))
- **`kde_window_background_blur`** option for KDE Plasma on Wayland. Thanks to @psomani16k. ([4d6b835](https://github.com/Dicklesworthstone/wezterm/commit/4d6b83585))
- **Font configuration for modals**: `char_select_font`, `command_palette_font`, `pane_select_font`. Thanks to @mgpinf. ([9606fa2](https://github.com/Dicklesworthstone/wezterm/commit/9606fa256))
- **`show_close_tab_button_in_tabs`** option for the fancy tab bar. Thanks to @zummenix. ([a46bad1](https://github.com/Dicklesworthstone/wezterm/commit/a46bad17d))
- **`PromptInputLine`** now supports optional `prompt` and `initial_value`. Thanks to @mgpinf and @ekorchmar. ([6a12dd1](https://github.com/Dicklesworthstone/wezterm/commit/6a12dd1a1), [b59cc5b](https://github.com/Dicklesworthstone/wezterm/commit/b59cc5b00))
- **`InputSelector`** label bg/fg colors and alphabet support. Thanks to @mgpinf. ([413ebd5](https://github.com/Dicklesworthstone/wezterm/commit/413ebd568))
- **Launcher label bg/fg** configuration options. Thanks to @mgpinf. ([4a32fea](https://github.com/Dicklesworthstone/wezterm/commit/4a32feaa7))
- **`TabInformation.is_last_active`** property. Thanks to @masriomarm. ([ad44b15](https://github.com/Dicklesworthstone/wezterm/commit/ad44b159c))
- **OSC 52** clipboard extension indicated in Primary DA response. Thanks to @j4james. ([58141ad](https://github.com/Dicklesworthstone/wezterm/commit/58141ad6f))
- **`wezterm record -o`** output file option. Thanks to @Tyarel8. ([195440e](https://github.com/Dicklesworthstone/wezterm/commit/195440e3f))
- **`ShowTabNavigator`** defaults to selecting active tab. Thanks to @mgpinf. ([f55293262](https://github.com/Dicklesworthstone/wezterm/commit/f55293262))
- X11: Drag and drop support for files, URLs, and text. Thanks to @ssiegel. ([b888c54](https://github.com/Dicklesworthstone/wezterm/commit/b888c547d))
- Search overlay now accepts IME composed input. Thanks to @kenchou. ([1533409](https://github.com/Dicklesworthstone/wezterm/commit/1533409ed))
- Search overlay supports arrow keys. Thanks to @Mrreadiness. ([72465af](https://github.com/Dicklesworthstone/wezterm/commit/72465af6d))
- Switched to the `nucleo` fuzzy matcher (matches `fzf` behavior more closely). ([b20c619](https://github.com/Dicklesworthstone/wezterm/commit/b20c61926))
- Serial support migrated to `serial2` crate. Thanks to @jeevithakannan2. ([b38b5ef](https://github.com/Dicklesworthstone/wezterm/commit/b38b5ef32))
- macOS: terminfo now compiled and bundled in the app bundle. Thanks to @ddeville. ([54af65d](https://github.com/Dicklesworthstone/wezterm/commit/54af65d2e))
- macOS: toast notifications switched to `UNUserNotificationCenter`. ([9abf85c](https://github.com/Dicklesworthstone/wezterm/commit/9abf85cab))
- macOS: Re-enabled Services menu. Thanks to @cpick. ([efec700](https://github.com/Dicklesworthstone/wezterm/commit/efec7001a))

### Bug Fixes

- macOS: Memory leak in MetalLayer management. Thanks to @I-Info. ([c8a6496](https://github.com/Dicklesworthstone/wezterm/commit/c8a649684))
- macOS: Fixed toast notifications not appearing as popups. ([d2fc835](https://github.com/Dicklesworthstone/wezterm/commit/d2fc83559))
- macOS: Prevented infinite loop in Services menu validation. Thanks to @cpick. ([cfb1e91](https://github.com/Dicklesworthstone/wezterm/commit/cfb1e9190))
- Wayland: Fixed fractional scaling issues in Hyprland 0.51.0. Thanks to @kalebo. ([c6f25ea](https://github.com/Dicklesworthstone/wezterm/commit/c6f25ea3f))
- Wayland: Fixed tiled and maximized window state issues. ([80b9ffd](https://github.com/Dicklesworthstone/wezterm/commit/80b9ffd14))
- Fix parsing of partial SGR mouse sequences. Thanks to @jgiannuzzi. ([30ef869](https://github.com/Dicklesworthstone/wezterm/commit/30ef869d7))
- Fix boundary check condition in renderstate. Thanks to @I-Info. ([bb29750](https://github.com/Dicklesworthstone/wezterm/commit/bb297506d))
- `max_fps` config can now be set to values > 255. Thanks to @beckend. ([598fb9c](https://github.com/Dicklesworthstone/wezterm/commit/598fb9ce1))
- Fix `SpawnCommand` ignoring environment variables and cwd. Thanks to @vincentbesanceney. ([0bb382b](https://github.com/Dicklesworthstone/wezterm/commit/0bb382bf5))
- Fix aliasing when using `Center` in `window_content_alignment`. Thanks to @juster-0. ([77d97d4](https://github.com/Dicklesworthstone/wezterm/commit/77d97d489))
- Fix deadlock in `domain_was_detached`. Thanks to @joexue. ([6688566](https://github.com/Dicklesworthstone/wezterm/commit/6688566ba))
- Fix `get_text_from_semantic_zone` missing last line. Thanks to @mgpinf. ([0f21892](https://github.com/Dicklesworthstone/wezterm/commit/0f21892bb))
- Fix `InputSelector` error when other overlay was active. Thanks to @mikkasendke. ([6e2eeed](https://github.com/Dicklesworthstone/wezterm/commit/6e2eeed6e))
- Fix hyperlinks with parentheses. Thanks to @psyclaudeZ. ([dd7ae90](https://github.com/Dicklesworthstone/wezterm/commit/dd7ae90f1))
- Improved handling of IBus IME via xcb-imdkit update. ([d4b50f6](https://github.com/Dicklesworthstone/wezterm/commit/d4b50f6cc))
- Long CSI sequences (> 16 params) now parsed correctly. Thanks to @jdugan6240. ([0f1c4a7](https://github.com/Dicklesworthstone/wezterm/commit/0f1c4a7fa))
- Fix panic when appending long runs to clusterline. ([7aadebf](https://github.com/Dicklesworthstone/wezterm/commit/7aadebfe4))
- Fix split behavior with `default_prog`. ([b11cd7a](https://github.com/Dicklesworthstone/wezterm/commit/b11cd7ae7))
- COPR build fixes for Fedora 43. ([712400a](https://github.com/Dicklesworthstone/wezterm/commit/712400a55))
- Fixed scrollback position issues in alt-screen mode. ([f911e48](https://github.com/Dicklesworthstone/wezterm/commit/f911e48a9))
- Fixed mouse multi-click requiring pixel precision. ([47b91d8](https://github.com/Dicklesworthstone/wezterm/commit/47b91d8e0))
- Fixed image aspect ratio with `Contain` mode. ([3a6c7a8](https://github.com/Dicklesworthstone/wezterm/commit/3a6c7a8c8))

### Fork-Specific

- License updated to MIT with OpenAI/Anthropic Rider. ([3caf998](https://github.com/Dicklesworthstone/wezterm/commit/3caf99851))
- GitHub social preview image added. ([7be950f](https://github.com/Dicklesworthstone/wezterm/commit/7be950fda))

---

## 20240203-110809-5046fc22

**Date**: 2024-02-03 | **Tag**: [`20240203-110809-5046fc22`](https://github.com/Dicklesworthstone/wezterm/commit/5046fc225992db6ba2ef8812743fadfdfe4b184a) | **Status**: Latest Stable Release

### Changed

- Default for `freetype_load_flags` is now `NO_HINTING` when DPI >= 100, otherwise `DEFAULT`.
- `wezterm -e` now waits for the spawned program to terminate. Thanks to @vimpostor.
- Reverted text cursor cell dimension change from the previous nightly.

### New

- Lua version displayed in the debug overlay. Thanks to @bbkane.
- `wezterm start --new-tab` and `wezterm connect --new-tab` for spawning into existing GUI windows; new `prefer_to_spawn_tabs` option.

### Fixed

- `freetype_load_flags = 'DEFAULT'` could not be specified.
- macOS: Fallback fonts could select thin or unspecified font attributes.
- Palette changes via escape sequences did not invalidate caches.
- Unix: Spawning a command using a relative path in a directory containing a directory of the same name failed.
- x11: Incorrect space key handling with `grp:win_space_toggle`.
- `wezterm set-working-directory` and `wezterm imgcat` did not apply tmux passthrough encoding.
- Tab bar did not immediately reflect `tab:set_title` changes.
- Command Palette: Missing space between keycaps on macOS.
- Command Palette: Showed default key assignments even when `disable_default_key_bindings` was set.

---

## 20240128-202157-1e552d76

**Date**: 2024-01-28 | **Tag**: [`20240128-202157-1e552d76`](https://github.com/Dicklesworthstone/wezterm/commit/1e552d764349522dabffeb240feb5b2728eff3d8)

### Changed

- Default for `front_end` reverted to `OpenGL`.
- Default for `freetype_load_flags` is now `NO_HINTING`.

### Fixed

- macOS: System LastResort font would be selected in preference to other fonts for bold fallback.
- Fancy tab bar took a few moments to update after closing a tab.
- Kitty Image Protocol: Fixed numerous issues. Thanks to @jonboh.

---

## 20240127-113634-bbcac864

**Date**: 2024-01-27 | **Tag**: [`20240127-113634-bbcac864`](https://github.com/Dicklesworthstone/wezterm/commit/bbcac86436fe31c98ad411ae880886619512fe94)

This is the first release where `front_end` defaults to `WebGpu`, and where
`pane.get_current_working_dir` returns a `Url` object instead of a string.

### Changed

- Default `front_end` is now `WebGpu`.
- `pane.get_current_working_dir` return type changed to `Url` object (**breaking for Lua configs**).
- CharSelect: Added emoji variations (skin tones), short codes section, improved fuzzy matching performance.
- `PaneSelect`: New modes `MoveToNewTab`, `MoveToNewWindow`, `SwapWithActiveKeepFocus`; `show_pane_ids` option.
- Nightly `.deb` packages now named `wezterm-nightly` (conflict with `wezterm`).
- `window_frame` colors now support alpha channel for transparent tab bar backgrounds.

### New

- `wezterm imgcat`: Added `--position`, `--no-move-cursor`, `--hold` options; tmux passthru support; automatic image resampling for large images.
- `pane:get_lines_as_escapes()` for capturing pane content with escape sequences preserved.
- Experimental COLR v1 font rasterization support via freetype and harfbuzz. Added `font_colr_rasterizer` option.
- Experimental SVG font support.
- `dpi_by_screen` config option for per-monitor DPI settings (macOS, X11, Wayland, Windows).
- `wezterm.gui.screens` now includes `effective_dpi` field.
- New `plugin_dir` field in `RepoSpec`.
- `wezterm imgcat`: Added `--resize` and `--resample` functionality.
- Over 80 new color schemes added.

### Fixed

- Disproportionate cursor rendering.
- Excessive calls to `format-tab-title` callback.
- Bar cursors not rendered on topmost layer.
- X11: Handling of high-speed key events.
- Compose key issues.
- `CloseCurrentPane` leaving a stranded pane in a tab.
- Strikethrough position for `line_height != 1.0`.
- Bitmap space glyphs with 0 size causing panic.
- X11/Wayland: AltGr handling in azerty and similar layouts.
- Mouse scroll on hovered but unfocused pane.
- Wayland resize increment calculations.

---

## 20230712-072601-f4abf8fd

**Date**: 2023-07-12 | **Tag**: [`20230712-072601-f4abf8fd`](https://github.com/Dicklesworthstone/wezterm/commit/f4abf8fde7d45ccdee443ea162b6bd23862b8e32)

### Changed

- `adjust_window_size_when_changing_font_size` default now depends on `tiling_desktop_environments`.
- Added eighth block corner glyphs to custom block glyphs. Thanks to @joouha.
- Mouse cursor no longer hidden when pressing only modifier keys.
- `PaneSelect` will un-zoom to show all panes, then re-zoom after.
- Background images now decoded asynchronously.
- CTRL-J/K added as launcher menu navigation alternatives.
- Duplicate CharSelect entries suppressed.
- `front_end` defaults to `OpenGL` again (reverted from WebGpu).

### New

- `window:keyboard_modifiers()` method.
- `win32_system_backdrop` option (Acrylic, Mica on Windows). Thanks to @kingavatar.
- `wezterm cli adjust-pane-size` command. Thanks to @mrjones2014.
- `augment-command-palette` event for extending the command palette.
- `char_select_bg_color` / `char_select_fg_color` options.
- Horizontal mouse wheel events now passed through. Thanks to @calops.
- `exit_behavior_messaging` option.
- `default_mux_server_domain` option.

### Fixed

- Modals did not respect alternative OS-level key maps.
- X11/Wayland: CTRL-key presses for non-US latin keymaps regression.
- Numerous kitty keyboard protocol issues (10+ issues resolved).
- Laggy behavior with continual serial data streams. Thanks to @pcc.
- `user-var-changed` event triggered for every GUI window instead of the correct one.
- SSH: Improved host key verification error messaging.
- Copy mode on Wayland.
- Plugin repository renaming on Windows.
- Nerd Font Symbols 3.0 breaking changes fallout (multiple rounds of fixes).
- `ClearToEndOfLine` in last column with `wrap_next=true`.
- Middle mouse drag on macOS.
- Divide by zero when resizing many tiny panes.
- Image display in `wezterm ssh`.
- Panic with corrupt webp files.
- Double-skewing behavior.
- Background image fallback scaling.
- `sftp::FilePermissions::is_readonly` reporting.
- `CopyMode(EditPattern)` not activating `search_mode` key table.

---

## 20230408-112425-69ae8472

**Date**: 2023-04-08 | **Tag**: [`20230408-112425-69ae8472`](https://github.com/Dicklesworthstone/wezterm/commit/69ae847273aa2b0a64bdb07cf19d3f6fbaaa6b71)

### Changed

- macOS: CTRL-modified keys no longer routed to IME by default; new `macos_forward_to_ime_modifier_mask` option.
- Multiplexer client can send locally configured color palette to mux server.
- Multiplexer: Closing a window with mux client panes now detaches the associated domain.
- `quick_select_patterns` and `hyperlink_rules` now support backreferences and look around assertions.
- `wezterm replay`: Added `--explain-only` and `--cat` options.

### New

- `PromptInputLine` action for prompting user for text input.
- `InputSelector` action for prompting user to select from a list.
- `pane:activate()` and `tab:activate()` methods.
- `ulimit_nofile` and `ulimit_nproc` options.
- `serial_ports` configuration for serial port access.
- `ssh_domains` auto-populated from `~/.ssh/config`.
- `display_pixel_geometry` option for subpixel antialiasing.
- Integrated title and tab bar with extensive customization. Thanks to @yuraiz.
- `wezterm cli set-tab-title`, `set-window-title`, `rename-workspace`, `get-pane-direction` commands.
- `pane:get_tty_name()` and `PaneInformation.tty_name`.
- `wezterm.has_action()` function.
- `wezterm cli zoom-pane` subcommand.

### Fixed

- Windows: Full screen mode messed up on config reload.
- macOS: Font size issue when spawning on external monitor.
- Freetype build issues.
- Wayland: Invalidation issues with WebGpu backend.
- Potential deadlock when using `wezterm cli split-pane`.

---

## 20230326-111934-3666303c

**Date**: 2023-03-26 | **Tag**: [`20230326-111934-3666303c`](https://github.com/Dicklesworthstone/wezterm/commit/3666303c7b26c6c966b3f136dbb954686d334cc3)

Bugfix release with documentation improvements for the `pane:move_to_xxx` methods.

---

## 20230320-124340-559cb7b0

**Date**: 2023-03-20 | **Tag**: [`20230320-124340-559cb7b0`](https://github.com/Dicklesworthstone/wezterm/commit/559cb7b0a72606a45673004ae7b5420268d843c6)

Major feature release introducing the Command Palette, Nix flake support, plugin system, and many Lua API additions.

### New

- **Command Palette**: `CTRL-SHIFT-P` activates the command palette.
- **Plugin system**: `wezterm.plugin` module for loading modules from git.
- **Nix flake** support for building WezTerm.
- `wezterm.config_builder()` for improved configuration ergonomics.
- `wezterm cli get-text` command for capturing pane content.
- `wezterm start --domain DOMAIN --attach` and `gui-attached` event.
- Copy Mode: `CTRL-U`/`CTRL-D` half-page scrolling, `MoveForwardWordEnd`, `CloseWithoutClear`.
- `window:focus()`, `ActivateWindow`, `ActivateWindowRelative` key assignments.
- `hide_mouse_cursor_when_typing` option.
- `pane:get_text_from_region()`, `pane:get_semantic_zones()`, `pane:get_semantic_zone_at()`.
- `tab:get_size()`, `tab:rotate_counter_clockwise()`.
- macOS: Dock menu for spawning new windows. Thanks to @dahlia.
- `quit_when_all_windows_are_closed` option.
- `wezterm.default_hyperlink_rules()` function.
- macOS: Association with `.command`, `.sh`, `.zsh`, `.bash`, `.fish`, `.tool` scripts.
- MuxDomain exposed to Lua with `wezterm.mux.get_domain()`, `all_domains()`, `set_default_domain()`.
- Many new color schemes.

### Fixed

- X11: Hanging or killing the IME could hang wezterm.
- SSH: `AddressFamily` option now respected.
- Windows: Panic when minimizing with `front_end='WebGpu'`.
- Font config: Failed to resolve certain clusters.
- `log_unknown_escape_sequences` config option added.
- SSH: Improved handling of dead sessions with automatic reconnection.
- Numlock and capslock interfering with mouse assignments.
- Wayland: Frame callback registration fix.
- OS keymapping issues with modals.

---

## 20221119-145034-49b9839f

**Date**: 2022-11-19 | **Tag**: [`20221119-145034-49b9839f`](https://github.com/Dicklesworthstone/wezterm/commit/49b9839fdb41eff4d2aae3bb91e6a4befb9cd20b)

### New

- `wezterm.color` module for working with colors programmatically.
- `wezterm.gui.get_appearance()` to detect dark mode.
- `wezterm.json_parse` and `wezterm.json_encode`.
- `wezterm.time` module with `call_after()`.
- `wezterm.procinfo` module.
- `window:set_position()`, `window:set_inner_size()`, `window:maximize()`, `window:restore()`, `window:toggle_fullscreen()`.
- `wezterm.gui.screens()` on all platforms.
- `window:get_selection_escapes_for_pane()`, `window:copy_to_clipboard()`.
- `update-status` event (deprecating `update-right-status`).
- ExecDomain support.
- Wayland: IME support via `zwp_text_input_v3`.
- `switch_to_last_active_tab_when_closing_tab` option.
- `CharSelect` modal for emoji/nerdfont/unicode input.
- `Pane::get_metadata` method.
- `user-var-changed` event.
- `max_fps` option (macOS, Windows).
- Pane border size and color config.
- `CopyMode('ClearSelectionMode')` action.
- `wezterm ls-fonts --rasterize-ascii --text foo`.
- `window:set_left_status()`.

### Fixed

- IME composition status applying to all panes.
- Hangul NFC shaping.
- Various search implementation correctness issues and performance (quadratic complexity fix).
- Blinking text and cursor invalidation.
- Tab rendering and cursor visibility issues.

---

## 20220905-102802-7d4b8249

**Date**: 2022-09-05 | **Tag**: [`20220905-102802-7d4b8249`](https://github.com/Dicklesworthstone/wezterm/commit/7d4b8249d7e481c7134f6d4aec527d99f772fa06)

Stabilization release following three pre-release candidates on August 7.

---

## 20220807-113146-c2fee766

**Date**: 2022-08-07 | **Tag**: [`20220807-113146-c2fee766`](https://github.com/Dicklesworthstone/wezterm/commit/c2fee766389cba16ee18a2680fdb8f9155956a3a)

Pre-release cycle with three candidate builds on the same day (`e2bf4683`, `56aa7133`, `608750d5`, `c2fee766`). The stable version is `c2fee766`.

---

## 20220624-141144-bd1b7c5d

**Date**: 2022-06-24 | **Tag**: [`20220624-141144-bd1b7c5d`](https://github.com/Dicklesworthstone/wezterm/commit/bd1b7c5d9a194b3932f86659e18e93c84649df65)

### New

- `wezterm show-keys` command.
- Kitty keyboard protocol support (`enable_kitty_keyboard`).
- `wezterm.GLOBAL` for persistent Lua state.
- `wezterm.mux` module with window spawning, pane splitting, and `gui-startup` / `mux-startup` events.
- `freetype_pcf_long_family_names` option.
- Win32 input mode: Extended/enhanced key concept.
- Line selection mode in copy mode.
- Drag and drop support for Wayland.

### Fixed

- X11: Copy and paste race condition between wezterm windows.
- Keyboard: Key up events in neovim with kitty keyboard protocol.
- Scrollbar thumb position for modified minimum thumb sizes.
- Dead keys handling on Windows and X11/Wayland.
- OSC 1337 `ReportCellSize` implementation.
- Automatic bold synthesis for fonts.

---

## 20220408-101518-b908e2dd

**Date**: 2022-04-08 | **Tag**: [`20220408-101518-b908e2dd`](https://github.com/Dicklesworthstone/wezterm/commit/b908e2dd8cd27c613c8d76e9e6abc5245e500d7d)

### New

- Experimental BiDi (bidirectional text) support via `experimental_bidi` config option.
- `swallow_mouse_click_on_window_focus` option.
- `wezterm.enumerate_ssh_hosts()` helper.
- Primary selection support on Wayland.
- DECSDM support.
- Separate `animation_fps` config for easing.

### Fixed

- Various underline and custom block glyph positioning.
- Iosevka font matching when multiple TTCs installed.
- Serial port busy waiting on Unix.
- Copying trailing whitespace from wrapped lines.
- UB in vtparse.
- Emoji width rendering regression.
- RTL text rendering.

---

## 20220319-142410-0fcdea07

**Date**: 2022-03-19 | **Tag**: [`20220319-142410-0fcdea07`](https://github.com/Dicklesworthstone/wezterm/commit/0fcdea07897a502234e9ce754436e859e7d7f172)

Bugfix release. Allowed slightly poorly formed sixel data to be rendered.

---

## 20220101-133340-7edc5b5a

**Date**: 2022-01-01 | **Tag**: [`20220101-133340-7edc5b5a`](https://github.com/Dicklesworthstone/wezterm/commit/7edc5b5ab088e7d34eb2f440648c6505f1966c61)

Two prior candidates on the same day (`f2c04077`, `e11b9460`). This is the stable release.

---

## 20211205-192649-672c1cc1

**Date**: 2021-12-05 | **Tag**: [`20211205-192649-672c1cc1`](https://github.com/Dicklesworthstone/wezterm/commit/672c1cc135f7d366f5bd2bd79a428b2f8b2282d1)

CI and packaging fix (Arch PKGBUILD conflict/provide).

---

## 20211204-082213-a66c61ee9

**Date**: 2021-12-04 | **Tag**: [`20211204-082213-a66c61ee9`](https://github.com/Dicklesworthstone/wezterm/commit/a66c61ee9912232f157968fb502f0e81559016a8)

### New

- Unicode version control escape sequence.
- `widechar_width` wcwidth implementation imported.
- `ssh_backend` config option; default switched to libssh.
- `canonicalize_pasted_newlines` option.
- `wezterm.background_child_process` function.
- `CommandBuilder::env_clear` and `CommandBuilder::env_remove`.
- Resize increments option (disabled by default).
- Improved fontconfig alias resolution (e.g., `monospace`).

### Fixed

- Reverse wraparound mode BS in last column.
- `APPIMAGE` and `APPDIR` environment variable propagation to child processes.
- Initial window size with fancy tab bar.
- Wayland: HiDPI initialization and frame title positioning.
- Wayland: Disabled key repeat support.
- Mux: Local/remote tab tracking.
- Zoomed pane exit terminating whole tab.

---

## 20210814-124438-54e29167

**Date**: 2021-08-14 | **Tag**: [`20210814-124438-54e29167`](https://github.com/Dicklesworthstone/wezterm/commit/54e29167ba25dd4b51bb4ceccfe92941b98d94e1)

### New

- `window_background_gradient` option with radial gradient support.
- Kitty image protocol: Basic animation support, source/offset options, deflate support.
- `add_to_config_reload_watch_list` for watching additional config files.
- Window state concept (fullscreen, maximized).
- Alternative alpha blending option.
- Additional eighth block custom glyphs.

### Fixed

- Off-by-one in line length computation.
- Transparency issues.
- Intel Mac background color rendering.
- Kitty image deletion by `image_id` without `placement_id`.

---

## 20210502-154244-3f7122cb

**Date**: 2021-05-02 | **Tag**: [`20210502-154244-3f7122cb`](https://github.com/Dicklesworthstone/wezterm/commit/3f7122cb3f9d2fd92fb836e2f4b2aa7b839b6c86)

### New

- Lua config system improvements.
- `wezterm.config_dir` and `wezterm.config_file` variables.

---

## 20210405-110924-a5bb5be8

**Date**: 2021-04-05 | **Tag**: [`20210405-110924-a5bb5be8`](https://github.com/Dicklesworthstone/wezterm/commit/a5bb5be80aa5e9e5ae49400507a43523ae2edf2d)

One prior pre-release (`20210404-111518-fe48951e`). Font system fixes for cross-platform compatibility.

---

## 20210314-114017-04b7cedd

**Date**: 2021-03-14 | **Tag**: [`20210314-114017-04b7cedd`](https://github.com/Dicklesworthstone/wezterm/commit/04b7cedd02eb533fa9eb3c3278764e982044ff96)

### Fixed

- Animated GIF playback.
- Full alpha blending for glyphs.

---

## 20210203-095643-70a364eb

**Date**: 2021-02-03 | **Tag**: [`20210203-095643-70a364eb`](https://github.com/Dicklesworthstone/wezterm/commit/70a364eb3fd5624d950f6ed87a239f97b00c6028)

### New

- `wezterm.terminfo` source for custom terminfo.
- Curly, dotted, dashed, and colored underline support.
- Underline color rendering.
- `bold_brightens_ansi_colors` option.
- `tab_max_width` option.
- `adjust_window_size_when_changing_font_size` option.
- Overline support.
- DECRQSS, DECRPTUI support.
- CBT (cursor backward tabulation) implementation.
- Non-24bpp display support improvements.

### Fixed

- Scroll wheel to cursor up/down mapping in alt screen.
- Texture space errors when scaling to large font sizes.
- X11 painting issues.
- Windows initial window size with display scaling != 100%.

---

## 20201101-103216-403d002d

**Date**: 2020-11-01 | **Tag**: [`20201101-103216-403d002d`](https://github.com/Dicklesworthstone/wezterm/commit/403d002d0a81d264b00611ee8f3f7591ad41b492)

### Fixed

- Potential crash with multiple EGL windows on Windows.
- Window resize behavior on Windows.

---

## 20200909-002054-4c9af461

**Date**: 2020-09-09 | **Tag**: [`20200909-002054-4c9af461`](https://github.com/Dicklesworthstone/wezterm/commit/4c9af4617961e8cad8ca682708f1dba08221d63c)

### New

- FreeBSD CI added.
- `alternate_buffer_wheel_scroll_speed` option.
- `default_cwd` option.
- `ClearScrollback` erase mode option.
- Dual font support.

---

## 20200718-095447-d2315640

**Date**: 2020-07-18 | **Tag**: [`20200718-095447-d2315640`](https://github.com/Dicklesworthstone/wezterm/commit/d23156403d1da7054b591aa75b08f16bb235a3e3)

Color scheme re-sync release.

---

## 20200620-160318-e00b076c

**Date**: 2020-06-20 | **Tag**: [`20200620-160318-e00b076c`](https://github.com/Dicklesworthstone/wezterm/commit/e00b076c81ba80b3dbe161d1a0a08a1773e87fcc)

### New

- F5+ key representations corrected.
- Hyperlink matching improvements for double-wide characters.
- CSI parameter documentation.
- DECSET 1047 support.

---

## 20200517-122836-92c201c6

**Date**: 2020-05-17 | **Tag**: [`20200517-122836-92c201c6`](https://github.com/Dicklesworthstone/wezterm/commit/92c201c6578bc16c5e881d07f51d7474a71fed94)

Named release with AppImage improvements.

---

## 20200503-171512-b13ef15f

**Date**: 2020-05-03 | **Tag**: [`20200503-171512-b13ef15f`](https://github.com/Dicklesworthstone/wezterm/commit/b13ef15f2833ac9af3b1cb8b9f19c985d0bdf410)

CI and documentation build improvements.

---

## 20200406-151651-5b700e4

**Date**: 2020-04-06 | **Tag**: [`20200406-151651-5b700e4`](https://github.com/Dicklesworthstone/wezterm/commit/5b700e4d5dfe831eb2e4d4d33f4d4119e9c37be0)

### New

- MS Terminal mode 25 cursor visibility support.

---

## 20200202-181957-765184e5

**Date**: 2020-02-02 | **Tag**: [`20200202-181957-765184e5`](https://github.com/Dicklesworthstone/wezterm/commit/765184e507f9959bb7f92b713b0f01db00ebbd4d)

Multiple build candidates on the same day. Included fixes for Windows tag builder and multi-line 2FA prompts.

---

## 20200113-214446-bb6251f

**Date**: 2020-01-13 | **Tag**: [`20200113-214446-bb6251f`](https://github.com/Dicklesworthstone/wezterm/commit/bb6251fad9e4baf658d7edb1a88fd7708e0acd6e)

### Fixed

- CLI proxy statistics printing interference.
- Changelog updates for this release.

---

## 20191218-101156-bf35707

**Date**: 2019-12-18 | **Tag**: [`20191218-101156-bf35707`](https://github.com/Dicklesworthstone/wezterm/commit/bf35707cec4f70538980b49602c2972c5fbbd3a9)

Documentation theme switched to "Hacker" (darker).

---

## 20191124-233250-cb9fd7d

**Date**: 2019-11-24 | **Tag**: [`20191124-233250-cb9fd7d`](https://github.com/Dicklesworthstone/wezterm/commit/cb9fd7db5b56d13ba782f3fd521ec19f1ce997b2)

### Fixed

- Tab bar setting respected on config reload.

---

## Earlier Releases (2019)

| Tag | Date | Commit | Notes |
|-----|------|--------|-------|
| `20190626-162911-cc019ce` | 2019-06-26 | [`cc019ce`](https://github.com/Dicklesworthstone/wezterm/commit/cc019ce5bc0d1efff3d7b1af0c0c8044957afd91) | Nightly build pipeline fixes |
| `20190623-095503-6999f15` | 2019-06-23 | [`6999f15`](https://github.com/Dicklesworthstone/wezterm/commit/6999f152047bf23542f1321e22b1f41e2254258a) | Nightly pre-release scheduling |
| `20190622-182319-23f8c9d` | 2019-06-22 | [`23f8c9d`](https://github.com/Dicklesworthstone/wezterm/commit/23f8c9d7e886ec2dad152c8b0b2070e697b7bf48) | Multiplexing documentation |
| `20190602-182820-c0e3bdf` | 2019-06-02 | [`c0e3bdf`](https://github.com/Dicklesworthstone/wezterm/commit/c0e3bdfb705d74ce7dfa0f2f563fb82d5454b952) | termwiz: spurious pipe wakeup fix |
| `20190520-072639-015a97e` | 2019-05-20 | [`015a97e`](https://github.com/Dicklesworthstone/wezterm/commit/015a97e2bebf4ef9d9044f98df3f4efb1f9fc312) | PTY example additions |
| `20190507-215356-073f9fb` | 2019-05-07 | [`073f9fb`](https://github.com/Dicklesworthstone/wezterm/commit/073f9fb1f8514f2ab41be629790e29be99e008fb) | AtomicUsize API update |
| `20190324-182322-1980e73` | 2019-03-24 | [`1980e73`](https://github.com/Dicklesworthstone/wezterm/commit/1980e738c732260ab3b50e2b0eb9bd2540d19a37) | First published release builds |

---

## Crate-Level Tags

These tags represent published versions of WezTerm's library crates on [crates.io](https://crates.io),
independent of the main application release cycle.

### termwiz (Terminal Widgets Library)

| Tag | Notes |
|-----|-------|
| `termwiz-0.23.3` / `termiz-0.23.3` | Latest; includes `wezterm-escape-parser` extraction, `no_std` support |
| `termwiz-0.23.2` / `termiz-0.23.2` | Bugfixes |
| `termwiz-0.23.1` / `termiz-0.23.1` | Bugfixes |
| `termwiz-0.23.0` | Added Cargo.lock, optional tmux/image features, `lazy_static` to `LazyLock` migration |
| `termwiz-0.22.0` | Concurrent with 20240127 release cycle |
| `termwiz-0.20.0` | Concurrent with 20221119 release |
| `termwiz-0.19.0` | Color types, bidi support |
| `termwiz-0.18.0` | Surface improvements |
| `termwiz-0.17.1` | winapi build fix |
| `termwiz-0.17.0` | Concurrent with 20220624 release cycle |
| `termwiz-0.16.0` | Concurrent with 20220408 release |
| `termwiz-0.13.0` | Concurrent with 20210502 release |
| `termwiz-0.12.0` | Concurrent with 20210404 release |
| `termwiz-0.9.0` | Concurrent with 20200505 release |
| `termwiz-0.8.0` | Concurrent with 20200406 release |
| `termwiz-0.7.0` / `termwiz-0.7.1` | Early 2020 |
| `termwiz-0.6.0` | Late 2019 |
| `termwiz-0.5.0` | Late 2019 |
| `termwiz-0.4.0` | Mid 2019 |
| `termwiz-0.3.0` / `termwiz-0.3.1` | Mid 2019 |
| `termwiz-0.2.0` | Early 2019 |
| `termwiz-0.1.0` | Initial release |

### vtparse (VT Parser)

| Tag | Notes |
|-----|-------|
| `vtparse-0.7.0` | `alloc` feature for `no_std`; CSI param limit increase to 256 |
| `vtparse-0.6.2` | Concurrent with 20220624 |
| `vtparse-0.6.1` | Concurrent with 20220408 |
| `vtparse-0.5.0` | Concurrent with 20210502 |

### portable-pty

| Tag | Notes |
|-----|-------|
| `portable-pty-0.8.1` | Concurrent with 20230320 |
| `portable-pty-0.8.0` | Serial2 migration |
| `portable-pty-0.4.0` | Concurrent with 20210502 |
| `portable-pty-0.3.0` | Concurrent with 20200505 |
| `portable-pty-0.2.0` | Late 2019 |

### Other Crate Tags

| Tag | Notes |
|-----|-------|
| `wezterm-ssh-0.3.0` | SSH improvements |
| `wezterm-ssh-0.2.0` | SSH agent support |
| `wezterm-ssh-0.1.0` / `0.1.1` | Initial SSH crate |
| `wezterm-bidi-0.1` | Bidirectional text support |
| `wezterm-color-types-0.1` | Color type extraction |
| `filedescriptor-0.8.1` | File descriptor utilities |

---

*This changelog was generated on 2026-03-21 from git history spanning 2019-03-24 to present, covering 33 tagged releases and 17 crate-level version tags.*
