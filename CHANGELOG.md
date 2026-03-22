# Changelog

All notable changes to [WezTerm](https://github.com/wezterm/wezterm) are documented here.
This file is maintained in the [Dicklesworthstone/wezterm](https://github.com/Dicklesworthstone/wezterm) fork.

WezTerm is a GPU-accelerated cross-platform terminal emulator and multiplexer written in Rust by [@wez](https://github.com/wez).
Releases are named using the pattern `YYYYMMDD-HHMMSS-commithash`.
The upstream project also publishes crate-level tags (e.g., `termwiz-0.23.3`) for its library components.

> For the upstream project's own detailed per-release notes, see <https://wezterm.org/changelog.html>.

---

## Table of Contents

- [Unreleased (Nightly)](#unreleased-nightly)
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

> 800+ commits on `main` since `20240203-110809-5046fc22`.

### Fork-Specific Changes

- License updated to MIT with OpenAI/Anthropic Rider. ([3caf998](https://github.com/Dicklesworthstone/wezterm/commit/3caf99851c5d3573a172324ce08bcc07405b6ce9))
- GitHub social preview image added. ([7be950f](https://github.com/Dicklesworthstone/wezterm/commit/7be950fda7371e964337b28515046cbcd71e56e3))
- `wa-integration` workspace feature flag for pane lifecycle observability. ([bb4dc15](https://github.com/Dicklesworthstone/wezterm/commit/bb4dc15c4e3d97ca580dce17c3dae7a286298ded), [c2c9598](https://github.com/Dicklesworthstone/wezterm/commit/c2c95983278eede8ba371171bdb891c73aac6cbd), [4f5f19d](https://github.com/Dicklesworthstone/wezterm/commit/4f5f19d3f7f45348fb54a6a35242ac977dd4c643))

### Rendering and GPU

- macOS: Migrated to `objc2` bindings for modern Objective-C interop. ([d3d9d8e](https://github.com/Dicklesworthstone/wezterm/commit/d3d9d8e39))
- macOS: Fixed memory leak in MetalLayer management. Thanks to @I-Info. ([c8a6496](https://github.com/Dicklesworthstone/wezterm/commit/c8a649684))
- macOS: Explicitly set window to sRGB colorspace to resolve incorrect colors on non-sRGB monitors. Thanks to @rianmcguire. ([d4b50f6](https://github.com/Dicklesworthstone/wezterm/commit/d4b50f6cc))
- Fixed boundary check condition in renderstate. Thanks to @I-Info. ([bb29750](https://github.com/Dicklesworthstone/wezterm/commit/bb297506d))
- Fixed pixel aliasing when using `window_content_alignment = "Center"`. Thanks to @juster-0. ([77d97d4](https://github.com/Dicklesworthstone/wezterm/commit/77d97d489))
- Fixed image aspect ratio with `Contain` mode. Thanks to @saltkid. ([3a6c7a8](https://github.com/Dicklesworthstone/wezterm/commit/3a6c7a8c8))
- Render invalidation issue when closing tabs other than the last tab. Thanks to @Mrreadiness. ([5441](https://github.com/wezterm/wezterm/pull/5441))
- Scrollback position incorrectly advanced in alt-screen mode. Thanks to @tbung and @loops. ([f911e48](https://github.com/Dicklesworthstone/wezterm/commit/f911e48a9))
- `max_fps` config now accepts values > 255. Thanks to @beckend. ([598fb9c](https://github.com/Dicklesworthstone/wezterm/commit/598fb9ce1))
- Upgraded wgpu from 24 to 25. ([b249d9c](https://github.com/Dicklesworthstone/wezterm/commit/b249d9c42))

### Terminal Emulation and Escape Sequences

- Indicate support for OSC 52 (clipboard extensions) in Primary DA response. Thanks to @j4james. ([58141ad](https://github.com/Dicklesworthstone/wezterm/commit/58141ad6f))
- Fixed parsing of partial/fragmented SGR mouse sequences. Thanks to @jgiannuzzi. ([30ef869](https://github.com/Dicklesworthstone/wezterm/commit/30ef869d7))
- Long CSI sequences (> 16 params) now parsed correctly. Thanks to @jdugan6240. ([0f1c4a7](https://github.com/Dicklesworthstone/wezterm/commit/0f1c4a7fa))
- Fixed panic when appending long runs to clusterline. ([7aadebf](https://github.com/Dicklesworthstone/wezterm/commit/7aadebfe4))
- CUP position parameters were mandatory when they should have been optional. Thanks to @wojciech-graj. ([6860](https://github.com/wezterm/wezterm/issues/6860))
- Panic when rewrapping very long lines. ([6729](https://github.com/wezterm/wezterm/issues/6729))

### tmux Control Mode (`tmux -CC`)

- **tmux -CC control mode** is now very usable: pane sync, tab/window management, split pane support, layout sync, performance improvements, and backward compatibility for older tmux versions. Thanks to @joexue. ([718d91a](https://github.com/Dicklesworthstone/wezterm/commit/718d91a62))
- Fixed tmux -CC mode not parsing output correctly. ([8fb60ad](https://github.com/Dicklesworthstone/wezterm/commit/8fb60ad82))
- Fixed tmux -CC error on `%config-error`. ([4ce83f0](https://github.com/Dicklesworthstone/wezterm/commit/4ce83f093))
- Fixed stack overflow on Windows when using tmux -CC. Thanks to @joexue. ([574e531](https://github.com/Dicklesworthstone/wezterm/commit/574e53190))

### Wayland

- **Major rewrite of the Wayland backend** using smithay-client-toolkit 0.18/0.19, contributed by @tzx, @tmccombs, @deviant, and many others. Includes proper window frame support, IME support, drag-and-drop, primary selection, tiled/maximized state handling, and better compositor compatibility.
- Fixed fractional scaling issues in Hyprland 0.51.0. Thanks to @kalebo. ([c6f25ea](https://github.com/Dicklesworthstone/wezterm/commit/c6f25ea3f))
- Fixed tiled and maximized window state issues. Thanks to @aliaksandr-trush. ([80b9ffd](https://github.com/Dicklesworthstone/wezterm/commit/80b9ffd14))
- Fixed potential panic on startup when monitors are hot-plugging. Thanks to @loops.
- Fixed `hide_cursor: Missing enter event serial` error. Thanks to @jmbaur.
- Fixed hang when launched under ChromeOS Crostini. Thanks to @dberlin.
- Fixed startup on Hyprland >= 0.37.0. Thanks to @fioncat.
- Improved startup performance on X11. Thanks to @blukai.

### SSH and Networking

- **SSH agent forwarding**: `wezterm ssh` and SSH multiplexer domains now support agent forwarding with automatic `SSH_AUTH_SOCK` maintenance. Thanks to @Riatre.
- SSH `ServerAliveInterval` now respected by libssh backend. ([909573f](https://github.com/Dicklesworthstone/wezterm/commit/909573fad))
- `ProxyUseFdPass` support. Thanks to @loops.
- wezterm-ssh now correctly handles two-phase processing of `%h` tokens. Thanks to @emc2314 and @wheatdog.
- wezterm-ssh now expands additional ssh_config tokens (`%d`, `%u`, `%L`). ([662e9b0](https://github.com/Dicklesworthstone/wezterm/commit/662e9b0c7))
- Fixed deadlock when a domain detaches due to SSH timeout. Thanks to @joexue.

### macOS

- macOS: toast notifications now use `UNUserNotificationCenter`; fixed notifications not appearing as popups. ([d2fc835](https://github.com/Dicklesworthstone/wezterm/commit/d2fc83559))
- Re-enabled macOS "Services" menu; prevented infinite loop in Services menu validation. Thanks to @cpick. ([efec700](https://github.com/Dicklesworthstone/wezterm/commit/efec7001a), [cfb1e91](https://github.com/Dicklesworthstone/wezterm/commit/cfb1e9190))
- Removed deprecated `NSWindowFullScreenButton`. ([85c587f](https://github.com/Dicklesworthstone/wezterm/commit/85c587f9f))
- Fixed notch avoidance padding in full screen mode. Thanks to @mbaird.
- Fixed wacky initial window size with external monitors or certain font sizes.

### Configuration and UI

- **`wezterm.serde` module**: JSON/TOML/YAML serialization and deserialization from Lua. Thanks to @expnn.
- **ConEmu progress reporting**: Preliminary support for ConEmu-style progress escape sequences via `pane:get_progress()`.
- **`window_content_alignment`** option to control excess pixel gap placement. Thanks to @Shiphan.
- **`Confirmation`** key assignment for prompts before destructive actions. Thanks to @mgpinf.
- **`MACOS_FORCE_SQUARE_CORNERS`** option for `window_decorations`. Thanks to @amadeusdotpng.
- **`MACOS_USE_BACKGROUND_COLOR_AS_TITLEBAR_COLOR`** option. Thanks to @Jay-Madden.
- **`macos_fullscreen_extend_behind_notch`** option. Thanks to @wryanzimmerman.
- **`QuickSelectArgs.skip_action_on_paste`** option. Thanks to @nhurlock.
- **`quick_select_remove_styling`** option for easier match spotting. Thanks to @mgpinf.
- **Unicode 16 octant characters** when `custom_block_glyphs` is enabled. Thanks to @eschnett.
- **Unicode Symbols for Legacy Computing** added to pixel-perfect block drawing glyphs. Thanks to @stribor14.
- **Git branch, progress bar, and spinner glyphs** added to `custom_block_glyphs`. Thanks to @BenBergman.
- **`cell_widths`** option for explicit cell width control. Thanks to @hamano.
- **`text_min_contrast_ratio`** and **`reverse_video_cursor_min_contrast`** options. Thanks to @jameshurst.
- **`kde_window_background_blur`** option for KDE Plasma on Wayland. Thanks to @psomani16k.
- **Font configuration for modals**: `char_select_font`, `command_palette_font`, `pane_select_font`. Thanks to @mgpinf.
- **`show_close_tab_button_in_tabs`** option for the fancy tab bar. Thanks to @zummenix.
- **`PromptInputLine`** now supports optional `prompt` and `initial_value`. Thanks to @mgpinf and @ekorchmar.
- **`InputSelector`** label bg/fg colors. Thanks to @mgpinf.
- **Launcher label bg/fg** configuration options. Thanks to @mgpinf.
- **`TabInformation.is_last_active`** property. Thanks to @masriomarm.
- **`ShowLauncherArgs`** customizable help text and alphabet option. Thanks to @mgpinf.
- **`ShowTabNavigator`** defaults to selecting the active tab. Thanks to @mgpinf.
- **`wezterm record -o`** output file option. Thanks to @Tyarel8.
- **`wezterm imgcat --hold`** now avoids local echo and accepts ESC/CTRL-C/CTRL-D to exit. Thanks to @mgpinf.
- Switched to the `nucleo` fuzzy matcher (matches `fzf` behavior more closely).
- Serial support migrated to `serial2` crate. Thanks to @jeevithakannan2.
- macOS: terminfo now compiled and bundled in the app bundle. Thanks to @ddeville.
- X11: drag and drop support for files, URLs, and text. Thanks to @ssiegel.
- Search overlay now accepts IME composed input and supports richer line editing. Thanks to @kenchou and @Mrreadiness.
- Upper bound of 999,999,999 for `scrollback_lines`. Thanks to @x3ro.
- Copy Mode `Close` action no longer implicitly scrolls to the bottom. Thanks to @LeszekSwirski.

### Architecture and Crate Refactoring

- **termwiz crate split**: Extracted `wezterm-cell`, `wezterm-char-props`, `wezterm-surface`, and `wezterm-escape-parser` as independent crates. The escape parser can now build in `no_std` environments.
- termwiz: Made tmux control code and image support optional via Cargo features.
- vtparse: Added `alloc` feature for `no_std` environments; CSI param limit increased to 256.
- RPM packaging split into `wezterm-common`, `wezterm-gui`, and `wezterm-mux-server` subpackages. ([b647d74](https://github.com/Dicklesworthstone/wezterm/commit/b647d74fd))

### Bug Fixes (Additional)

- Fixed `SpawnCommand` ignoring environment variables and cwd. Thanks to @vincentbesanceney.
- Fixed deadlock in `domain_was_detached`. Thanks to @joexue.
- Fixed `get_text_from_semantic_zone` missing last line. Thanks to @mgpinf.
- Fixed `InputSelector` error when other overlay was active. Thanks to @mikkasendke.
- Fixed hyperlinks with parentheses. Thanks to @psyclaudeZ.
- Improved handling of IBus IME via xcb-imdkit update.
- Fixed split behavior with `default_prog`. ([b11cd7a](https://github.com/Dicklesworthstone/wezterm/commit/b11cd7ae7))
- Fixed mouse multi-click requiring pixel precision. Thanks to @jbiosca78.
- Race condition when quickly adjusting font scale. Thanks to @jknockel.
- The bell would ring each window instead of just the correct one. Thanks to @loops.
- X11: transient errors in obtaining/setting the selection could cause wezterm to exit. Thanks to @loops.
- Multiplexer: could lose track of delta updates if the display changed during computation. Thanks to @loops.
- Quick select mode now accepts unix paths with `//` in them.
- Blob leases for image rendering could be removed by temp directory cleaners; now stored in cache dir.
- We now respect line wrapping in alt-screen mode. Thanks to @eternity74.
- DECSLRM incorrectly clamped the left margin based on terminal height instead of width. Thanks to @j4james and @tmccombs.
- macOS: Key repeat would stop when switching between held keys when `use_ime` was enabled. Thanks to @psyclaudeZ.
- `wezterm cli split-pane --move-pane-id` could kill panes. Thanks to @scauligi.
- Glyph rendering fix for custom_block_glyphs. Thanks to @bew.
- `pane:current_working_dir.file_path` returned incorrect results for paths with `#` or `?`. Thanks to @loops.
- Plugins: normalized the plugin path to exclude trailing slashes. Thanks to @joncrangle.
- Zooming a tab might not work if you also recently used `pane:activate()`. Thanks to @SpyMachine.

### Dependency Updates

- Bundled harfbuzz to 11.2.1, freetype to 2.13.3, libssh to 0.11.1
- Bundled conpty.dll and OpenConsole.exe to build 1.22.250204002.nupkg
- Bundled Nerd Font Symbols font to v3.3.0, Noto Color Emoji to 2.047
- image crate to 0.25 (JPEG images now decoded via zune-jpeg)
- 30+ new color schemes added (Astrodark, Eldritch, Vesper, Everforest variants, and more)

---

## 20240203-110809-5046fc22

**Date**: 2024-02-03 | **Tag**: [`20240203-110809-5046fc22`](https://github.com/Dicklesworthstone/wezterm/commit/5046fc225992db6ba2ef8812743fadfdfe4b184a) | **Status**: Latest Stable Release

### Font Rendering

- Default for `freetype_load_flags` is now `NO_HINTING` when DPI >= 100, otherwise `DEFAULT`. ([#4902](https://github.com/wezterm/wezterm/issues/4902))
- Fixed: it was not possible to specify `freetype_load_flags = 'DEFAULT'`.
- macOS: fallback fonts could select thin or unspecified font attributes. ([#4808](https://github.com/wezterm/wezterm/issues/4808))

### CLI and Window Management

- `wezterm -e` now waits for the spawned program to terminate. Thanks to @vimpostor. ([#4535](https://github.com/wezterm/wezterm/issues/4535))
- `wezterm start --new-tab` and `wezterm connect --new-tab` for spawning into existing GUI windows; new `prefer_to_spawn_tabs` option.
- Lua version displayed in the debug overlay. Thanks to @bbkane. ([#4943](https://github.com/wezterm/wezterm/issues/4943))
- Reverted the text cursor cell dimension change from the previous nightly. ([#2882](https://github.com/wezterm/wezterm/issues/2882))

### Bug Fixes

- Palette changes via escape sequences did not invalidate caches. ([#4932](https://github.com/wezterm/wezterm/issues/4932))
- Unix: spawning a command using a relative path in a directory containing a directory of the same name failed. ([#4920](https://github.com/wezterm/wezterm/issues/4920))
- X11: incorrect space key handling with `grp:win_space_toggle`. ([#4910](https://github.com/wezterm/wezterm/issues/4910))
- `wezterm set-working-directory` and `wezterm imgcat` did not apply tmux passthrough encoding. ([#4940](https://github.com/wezterm/wezterm/issues/4940))
- Tab bar did not immediately reflect `tab:set_title` changes. ([#4941](https://github.com/wezterm/wezterm/issues/4941))
- Command Palette: missing space between keycaps on macOS. ([#4885](https://github.com/wezterm/wezterm/issues/4885))
- Command Palette: showed default key assignments even when `disable_default_key_bindings` was set. ([#4724](https://github.com/wezterm/wezterm/issues/4724))

---

## 20240128-202157-1e552d76

**Date**: 2024-01-28 | **Tag**: [`20240128-202157-1e552d76`](https://github.com/Dicklesworthstone/wezterm/commit/1e552d764349522dabffeb240feb5b2728eff3d8)

### Changed

- Default for `front_end` reverted to `OpenGL`.
- Default for `freetype_load_flags` is now `NO_HINTING`. ([#4874](https://github.com/wezterm/wezterm/issues/4874))

### Fixed

- macOS: System LastResort font would be selected in preference to other fonts for bold fallback. ([#4877](https://github.com/wezterm/wezterm/issues/4877))
- Fancy tab bar took a few moments to update after closing a tab. ([#4880](https://github.com/wezterm/wezterm/issues/4880))
- Kitty Image Protocol: fixed numerous issues. Thanks to @jonboh. ([#1156](https://github.com/wezterm/wezterm/issues/1156), [#2084](https://github.com/wezterm/wezterm/issues/2084), [#3918](https://github.com/wezterm/wezterm/issues/3918), [#4847](https://github.com/wezterm/wezterm/issues/4847))

---

## 20240127-113634-bbcac864

**Date**: 2024-01-27 | **Tag**: [`20240127-113634-bbcac864`](https://github.com/Dicklesworthstone/wezterm/commit/bbcac86436fe31c98ad411ae880886619512fe94)

This is the first release where `front_end` defaults to `WebGpu`, and where `pane.get_current_working_dir` returns a `Url` object instead of a string.

### GPU and Rendering

- Default `front_end` is now `WebGpu` (Metal, Vulkan, DX 12 drivers).
- Experimental COLR v1 font rasterization support via freetype and harfbuzz; `font_colr_rasterizer` option.
- Experimental SVG font support.
- `dpi_by_screen` config option for per-monitor DPI settings (macOS, X11, Wayland, Windows).
- `wezterm.gui.screens` now includes `effective_dpi` field.
- Bar cursors now rendered above text. Thanks to @ErrorNoInternet.
- Bundled harfbuzz to 8.3.0, freetype to 2.13.1, wgpu to 0.18.
- Bundled Nerd Font Symbols font to v3.1.1, Noto Color Emoji to 2.038.

### Configuration and Lua API

- **Breaking**: `pane.get_current_working_dir` return type changed to `Url` object.
- CharSelect: added emoji variations (skin tones), short codes section, improved fuzzy matching.
- `PaneSelect`: new modes `MoveToNewTab`, `MoveToNewWindow`, `SwapWithActiveKeepFocus`; `show_pane_ids` option.
- `window_frame` colors now support alpha channel for transparent tab bar backgrounds.
- `notification_handling` to control notification suppression based on focus.
- `command_palette_rows` to control command palette row count. Thanks to @exastone.
- `ToggleAlwaysOnTop`, `ToggleAlwaysOnBottom`, `SetWindowLevel` key assignments (macOS). Thanks to @rawnly.
- Double click on tab bar to toggle maximize/normal state. Thanks to @junnplus.
- `freetype_load_target` / `freetype_render_target` now support `VerticalLcd`. Thanks to @xiaopengli89.
- `CTRL-[` closes the launcher menu.
- `InputSelector` enhanced with alphabet for quick launch beyond 10 items. Thanks to @Danielkonge.
- `pane:get_lines_as_escapes()` for capturing content with escape sequences preserved.

### CLI Improvements

- `wezterm imgcat`: added `--position`, `--no-move-cursor`, `--hold` options; tmux passthru support; automatic image resampling for large images.
- `wezterm cli zoom-pane`. Thanks to @quantonganh.
- Nightly `.deb` packages now named `wezterm-nightly` (conflict with `wezterm`).
- 80+ new color schemes added.

### Bug Fixes

- Disproportionate cursor rendering. ([#2882](https://github.com/wezterm/wezterm/issues/2882))
- Excessive calls to `format-tab-title` callback. Thanks to @crides.
- X11: handling of high-speed key events. ([#4615](https://github.com/wezterm/wezterm/issues/4615))
- `CloseCurrentPane` leaving a stranded pane in a tab.
- Strikethrough position for `line_height != 1.0`.
- X11/Wayland: AltGr handling in azerty and similar layouts.
- Mouse scroll on hovered but unfocused pane.
- Wayland: wezterm wouldn't start on Plasma 6 or newer sway. Thanks to @hexchain.
- Windows: crash on Windows 11 DX 12 with WebGpu frontend.
- macOS: Leak of NSWindow and NSView objects. Thanks to @0f-0b.
- Initial G1 state non-conformance. Thanks to @ninjalj.
- Correct `SUPER` modifier handling in kitty protocol. Thanks to @gabyx.
- macOS: honor `window_close_confirmation` when quitting. Thanks to @quantonganh.
- Incorrect sunset/sunrise progression. Thanks to @mikyk10.
- Scrolling glitch in Command Palette, Launcher, and InputSelector. Thanks to @metiftikci.
- `use_resize_increments` now accounts for window padding. Thanks to @jknockel.
- F13-F24 keys now supported. Thanks to @ovidiu-ionescu.
- Gogh color schemes had incorrect cursor foreground color.

---

## 20230712-072601-f4abf8fd

**Date**: 2023-07-12 | **Tag**: [`20230712-072601-f4abf8fd`](https://github.com/Dicklesworthstone/wezterm/commit/f4abf8fde7d45ccdee443ea162b6bd23862b8e32)

### Configuration

- `adjust_window_size_when_changing_font_size` default now depends on `tiling_desktop_environments`.
- `PaneSelect` will now un-zoom to show all panes, then re-zoom.
- Background images now decoded asynchronously.
- `front_end` defaults to `OpenGL` again (reverted from WebGpu).
- CTRL-J/K added as launcher menu navigation alternatives.
- Duplicate CharSelect entries suppressed. Thanks to @vimpostor.
- Mouse cursor no longer hidden when pressing only modifier keys.

### New Features

- `window:keyboard_modifiers()` method.
- `win32_system_backdrop` option (Acrylic, Mica on Windows). Thanks to @kingavatar.
- `wezterm cli adjust-pane-size` command. Thanks to @mrjones2014.
- `augment-command-palette` event for extending the command palette.
- `char_select_bg_color` / `char_select_fg_color` options. Thanks to @junnplus.
- Horizontal mouse wheel events now passed through. Thanks to @calops.
- `exit_behavior_messaging` option.
- `default_mux_server_domain` option.

### Bug Fixes

- Modals did not respect alternative OS-level key maps.
- X11/Wayland: CTRL-key presses for non-US latin keymaps regression.
- Numerous kitty keyboard protocol issues (10+ issues resolved).
- Laggy behavior with continual serial data streams. Thanks to @pcc.
- `user-var-changed` event triggered for every GUI window instead of the correct one.
- SSH: improved host key verification error messaging.
- Nerd Font Symbols 3.0 breaking changes (multiple rounds of fixes).
- Divide by zero when resizing many tiny panes.
- Image display in `wezterm ssh`.
- Panic with corrupt webp files.
- macOS: hang or crash when copying or right clicking the about dialog.
- `CopyMode("EditPattern")` not activating `search_mode` key table.

### Dependency Updates

- Bundled harfbuzz to 8.0.1, freetype to 2.13.0, Nerd Font Symbols to v3.0.1.
- 200+ new color schemes imported from Gogh, terminal.sexy, and other sources.

---

## 20230408-112425-69ae8472

**Date**: 2023-04-08 | **Tag**: [`20230408-112425-69ae8472`](https://github.com/Dicklesworthstone/wezterm/commit/69ae847273aa2b0a64bdb07cf19d3f6fbaaa6b71)

### Terminal Interaction

- `PromptInputLine` action for prompting user for text input.
- `InputSelector` action for prompting user to select from a list.
- `pane:activate()` and `tab:activate()` methods.
- `wezterm.has_action()` for cross-version config compatibility.
- `wezterm cli set-tab-title`, `set-window-title`, `rename-workspace`, `get-pane-direction` commands.
- `pane:get_tty_name()` and `PaneInformation.tty_name`.
- `wezterm cli zoom-pane` subcommand.

### UI and Appearance

- Integrated title and tab bar with extensive customization. Thanks to @yuraiz.
- `display_pixel_geometry` option for subpixel antialiasing.
- macOS: CTRL-modified keys no longer routed to IME by default; new `macos_forward_to_ime_modifier_mask` option.
- `quick_select_patterns` and `hyperlink_rules` now support backreferences and look-around assertions.

### System Administration

- `ulimit_nofile` and `ulimit_nproc` options.
- `serial_ports` configuration for serial port access.
- `ssh_domains` auto-populated from `~/.ssh/config`.
- `wezterm replay` new `--explain-only` and `--cat` options.

### Bug Fixes

- Windows: full screen mode messed up on config reload.
- macOS: font size issue when spawning on external monitor.
- Wayland: invalidation issues with WebGpu backend.
- Potential deadlock when using `wezterm cli split-pane`.
- Multiplexer: stale remote window mapping, pane size issues, config propagation.

---

## 20230326-111934-3666303c

**Date**: 2023-03-26 | **Tag**: [`20230326-111934-3666303c`](https://github.com/Dicklesworthstone/wezterm/commit/3666303c7b26c6c966b3f136dbb954686d334cc3)

### New

- `mouse_wheel_scrolls_tabs` option. Thanks to @eaglgenes101.
- `wezterm cli kill-pane`, `activate-pane`, `activate-tab` commands.
- macOS: `macos_window_background_blur` for translucent window effect. Thanks to @Gkirito.
- `new-tab-button-click` event for overriding the New Tab button behavior.
- `pane:move_to_new_window()`, `pane:move_to_new_tab()`.

### Bug Fixes

- SSH ProxyCommand didn't parse commands containing `=` correctly.
- `wezterm --skip-config` produced `unexpected argument` error.
- ConPTY: logical line wrapping falsely joining long runs of output.
- `wezterm cli activate-pane-direction` didn't cause GUI repaint.
- imgcat broken with multiplexer protocol.
- macOS: command line parameters beyond the first opened spurious windows.
- `CTRL-SHIFT-R` in CharSelect mode performed global `ReloadConfiguration` instead.
- Lingering openconsole.exe processes on Windows. Thanks to @mbikovitsky.

---

## 20230320-124340-559cb7b0

**Date**: 2023-03-20 | **Tag**: [`20230320-124340-559cb7b0`](https://github.com/Dicklesworthstone/wezterm/commit/559cb7b0a72606a45673004ae7b5420268d843c6)

Major feature release introducing the Command Palette, Nix flake support, plugin system, and many Lua API additions.

### Headline Features

- **Command Palette**: `CTRL-SHIFT-P` activates the command palette.
- **Plugin system**: `wezterm.plugin` module for loading modules from git.
- **Nix flake** support for building WezTerm.
- `wezterm.config_builder()` for improved configuration ergonomics.
- `wezterm cli get-text` command for capturing pane content.

### Lua API Additions

- `window:focus()`, `ActivateWindow`, `ActivateWindowRelative` key assignments.
- `pane:get_text_from_region()`, `pane:get_semantic_zones()`, `pane:get_semantic_zone_at()`.
- `tab:get_size()`, `tab:rotate_counter_clockwise()`.
- MuxDomain exposed to Lua with `wezterm.mux.get_domain()`, `all_domains()`, `set_default_domain()`.
- `wezterm.default_hyperlink_rules()` function.
- `hide_mouse_cursor_when_typing` option. Thanks to @ProspectPyxis.
- `quit_when_all_windows_are_closed` option.

### Copy Mode

- `CTRL-U`/`CTRL-D` half-page scrolling. Thanks to @pengux.
- `MoveForwardWordEnd` and `CloseWithoutClear` actions.
- Home and End default assignments. Thanks to @cunha.

### Platform

- macOS: dock menu for spawning new windows. Thanks to @dahlia.
- macOS: association with `.command`, `.sh`, `.zsh`, `.bash`, `.fish`, `.tool` scripts.
- macOS: initial cut at native menu bar.
- `wezterm start` now accepts `--domain` and `--attach` options.
- `wezterm -e` is now an alias for `wezterm start`. Thanks to @Abdiramen.

### Bug Fixes

- X11: hanging or killing the IME could hang wezterm.
- SSH: `AddressFamily` option now respected.
- Windows: panic when minimizing with `front_end='WebGpu'`.
- Font config: failed to resolve certain clusters.
- SSH: improved handling of dead sessions with automatic reconnection.
- Numlock and capslock interfering with mouse assignments.
- Wayland: frame callback registration fix.
- OS keymapping issues with modals.
- `bold_brightens_ansi_colors` now supports `"BrightOnly"`.
- Config warnings now shown in the configuration error window.
- `wezterm.GLOBAL` now returns references rather than copies.
- Many more fixes spanning X11, Wayland, multiplexer, and search.

---

## 20221119-145034-49b9839f

**Date**: 2022-11-19 | **Tag**: [`20221119-145034-49b9839f`](https://github.com/Dicklesworthstone/wezterm/commit/49b9839fdb41eff4d2aae3bb91e6a4befb9cd20b)

### Performance

- Reduced CPU and RAM utilization; reduced overhead of parsing output and rendering to the GPU.
- Internal scrollback storage improvements reduce per-cell overhead by up to ~40x.
- Improved search performance (quadratic complexity fix).
- Compensated for TUI program flicker via `mux_output_parser_coalesce_delay_ms`.

### Lua API

- `wezterm.color` module for working with colors programmatically.
- `wezterm.gui.get_appearance()` for dark mode detection.
- `wezterm.json_parse` and `wezterm.json_encode`.
- `wezterm.time` module with `call_after()`.
- `wezterm.procinfo` module for local process information.
- `window:set_position()`, `window:set_inner_size()`, `window:maximize()`, `window:restore()`, `window:toggle_fullscreen()`.
- `window:get_selection_escapes_for_pane()`, `window:copy_to_clipboard()`.
- `window:is_focused()` method; `window-focus-changed` event.
- `pane:inject_output`, `pane:is_alt_screen_active()`.
- `ResetTerminal` key assignment.
- `SetPaneZoomState` key assignment.
- `wezterm.gui.default_key_tables` and `wezterm.gui.default_keys`.
- `wezterm.gui.screens()` on all platforms.

### Terminal Emulation

- `update-status` event (deprecating `update-right-status`).
- `ExecDomain` support.
- Wayland: IME support via `zwp_text_input_v3`.
- `modifyOtherKeys` keyboard encoding support.
- Superscript and subscript text attributes via SGR 73/74.
- `CharSelect` modal for emoji/nerdfont/unicode input (CTRL-SHIFT-U).
- `user-var-changed` event.
- `max_fps` option (macOS, Windows).
- Pane border size and color config.
- `CopyMode('ClearSelectionMode')` action.
- `front_end = "WebGpu"` enables Metal, Vulkan and DX 12 drivers.
- `normalize_output_to_unicode_nfc` option.
- `cursor_thickness`, `underline_thickness`, `underline_position`, `strikethrough_position` options.
- Utf8 mouse reporting (DECSET 1005).
- `window:set_left_status()`.
- `wezterm ls-fonts --rasterize-ascii --text foo`.

### Bug Fixes

- 40+ bugs fixed across IME composition, search, blinking text, tab rendering, Wayland pasting, X11 crashes, charselect, and more. See upstream changelog for full details.
- Hundreds of new color schemes imported from base16, Gogh, and terminal.sexy.

---

## 20220905-102802-7d4b8249

**Date**: 2022-09-05 | **Tag**: [`20220905-102802-7d4b8249`](https://github.com/Dicklesworthstone/wezterm/commit/7d4b8249d7e481c7134f6d4aec527d99f772fa06)

Stabilization release following three pre-release candidates on August 7.

- `switch_to_last_active_tab_when_closing_tab` option.
- fontconfig: allow matching non-monospace fonts for fallback when no monospace fonts found.
- `os.getenv` now resolves environment variables that require re-login to update.
- Searching is now incremental and shows progress.
- Fixed Hangul in NFD incorrectly shaped; visual artifacts when resizing splits.

Prior build candidates on this date: `20220807-081338-e2bf4683`, `20220807-093823-56aa7133`, `20220807-105216-608750d5`, `20220807-113146-c2fee766`, `20220903-194523-3bb1ed61`, `20220904-064125-9a6cee2b`.

---

## 20220807-113146-c2fee766

**Date**: 2022-08-07 | **Tag**: [`20220807-113146-c2fee766`](https://github.com/Dicklesworthstone/wezterm/commit/c2fee766389cba16ee18a2680fdb8f9155956a3a)

### Headline

- `CharSelect` modal (CTRL-SHIFT-U) for emoji/unicode/nerdfont character picking with fuzzy search.
- `window_frame` border size and color configuration.
- `max_fps` option.
- Cursor changes to lock glyph for password input detection.
- `CopyMode` now supports selecting and moving by semantic zones.
- `user-var-changed` event for Lua scripting.
- `colors` now override `color_scheme` (previously mutually exclusive).

### New Color Schemes

- carbonfox, Dracula (Official), Poimandres, Sequoia Monochrome/Moonlight, SynthwaveAlpha, and more.

### Bug Fixes

- macOS crash on startup if `$SHELL` points to non-executable.
- Tab titles truncated too short.
- IME composition in `wezterm ssh`.
- `x` and `+` buttons in fancy tab bar now always square.
- Middle click tab close now confirms.
- Mouse cursor reset to arrow when leaving window.

---

## 20220624-141144-bd1b7c5d

**Date**: 2022-06-24 | **Tag**: [`20220624-141144-bd1b7c5d`](https://github.com/Dicklesworthstone/wezterm/commit/bd1b7c5d9a194b3932f86659e18e93c84649df65)

### Headline Features

- `background` option for rich background compositing and parallax scrolling effects.
- Kitty keyboard protocol support (`enable_kitty_keyboard`).
- `wezterm.GLOBAL` for persistent Lua state across config reloads.
- `wezterm.mux` module with `gui-startup` / `mux-startup` events for startup arrangement.
- `wezterm show-keys` command.
- Copy Mode and Search Mode key assignments now configurable.
- `PaneSelect` key assignment.
- `SplitPane` and `RotatePanes` key assignments.
- ALT+drag for rectangular block selection; SHIFT-v for line selection in Copy Mode.
- `cell_width` option.
- Drag and drop support on macOS, Windows, and Wayland.
- win32-input-mode enabled by default on Windows.
- Wayland: `enable_wayland` now defaults to `true`.
- `exit_behavior` now defaults to `"Close"`.

### Bug Fixes

- 30+ fixes spanning macOS keyboard handling, X11 event processing, Wayland pasting, multiplexer sessions, deadkey handling, and scroll issues.
- Bundled harfbuzz to 4.3.0.

---

## 20220408-101518-b908e2dd

**Date**: 2022-04-08 | **Tag**: [`20220408-101518-b908e2dd`](https://github.com/Dicklesworthstone/wezterm/commit/b908e2dd8cd27c613c8d76e9e6abc5245e500d7d)

### New

- **Key Tables** feature for powerful modal key assignments.
- Experimental BiDi (bidirectional text) support via `experimental_bidi`.
- `swallow_mouse_click_on_window_focus` option.
- `wezterm.enumerate_ssh_hosts()` helper.
- Primary selection support on Wayland.
- DECSDM support.
- Separate `animation_fps` config for easing.
- Kitty Image Protocol: shared memory data transmission. Thanks to @tantei3.
- Per-fallback font scaling.
- Sixel parsing performance improvements.
- macOS: fullscreen mode now avoids the notch.

### Bug Fixes

- Various underline and custom block glyph positioning.
- Iosevka font matching with multiple TTCs.
- Serial port busy waiting on Unix.
- Multiplexer performance with images.
- `CloseCurrentPane{confirm=false}` phantom tabs.
- 20+ additional fixes.

---

## 20220319-142410-0fcdea07

**Date**: 2022-03-19 | **Tag**: [`20220319-142410-0fcdea07`](https://github.com/Dicklesworthstone/wezterm/commit/0fcdea07897a502234e9ce754436e859e7d7f172)

### Headline Features

- **Key assignments now use Physical Key locations by default** (new `key_map_preference` option to revert).
- Bundled `Symbols Nerd Font Mono` as default fallback font for all Nerd Fonts glyphs.
- **Workspaces**: `SwitchToWorkspace`, `SwitchWorkspaceRelative`, `default_workspace`, `window:active_workspace()`.
- `ShowLauncherArgs` key assignment.
- SGR-Pixels mouse reporting.
- win32-input-mode for ConPTY (opt-in).
- `wezterm cli send-text` command.
- `wezterm.nerdfonts` Lua module.
- `window_padding` now accepts `"1cell"` or `"30%"` values.
- `canonicalize_pasted_newlines` option.
- `wezterm.enumerate_ssh_hosts()` for auto-generating ssh domain config.
- `pane:has_unseen_output()` for marking tabs with unseen output.
- Nautilus context menu extension. Thanks to @lunaryorn.
- Primary selection on Wayland.
- Blinking text/cursor now eased rather than binary-blinked.
- `IME` and dead key composition state shown inline using terminal font.

### Bug Fixes

- 30+ fixes including fontconfig alias resolution, CTRL+C on non-latin layouts, ligatured glyph rendering, window resize issues, Korean NFD text, Wayland input after suspend/resume, and many X11/macOS-specific issues.

---

## 20220101-133340-7edc5b5a

**Date**: 2022-01-01 | **Tag**: [`20220101-133340-7edc5b5a`](https://github.com/Dicklesworthstone/wezterm/commit/7edc5b5ab088e7d34eb2f440648c6505f1966c61)

Two prior candidates on the same day (`f2c04077`, `e11b9460`).

### Headline

- **Fancy Tab Bars** are now the default.
- **Kitty Image Protocol** support enabled by default.
- `ScrollToTop`, `ScrollToBottom`, `ActivateTabRelativeNoWrap`, `QuickSelectArgs` key assignments.
- `wezterm.open_with` function.
- `wezterm.get_builtin_color_schemes()` function.
- `pane:get_foreground_process_name()`, `pane:get_current_working_dir` (Windows support without OSC 7).
- `ActivatePaneDirection` supports `"Next"` and `"Prev"`.
- `pane:get_logical_lines_as_text`.
- SSH Domains `ssh_config` overrides.
- `default_gui_startup_args`, `mux-is-process-stateful` event.
- Per-font `harfbuzz_features`, `freetype_load_target`, `freetype_load_flags`.
- HSL colorspace support.
- Colors: Alabaster, CGA, MaterialDesignColors, darkermatrix, nord-light.

### Bug Fixes

- 30+ fixes including gamma correction, Korean text sizing, split pane transparency, Wayland HiDPI, font matching, and multiplexer session tracking.

---

## 20211205-192649-672c1cc1

**Date**: 2021-12-05 | **Tag**: [`20211205-192649-672c1cc1`](https://github.com/Dicklesworthstone/wezterm/commit/672c1cc135f7d366f5bd2bd79a428b2f8b2282d1)

CI and packaging fix (Windows PTY handles, Arch PKGBUILD conflicts, update notification reliability).

---

## 20211204-082213-a66c61ee9

**Date**: 2021-12-04 | **Tag**: [`20211204-082213-a66c61ee9`](https://github.com/Dicklesworthstone/wezterm/commit/a66c61ee9912232f157968fb502f0e81559016a8)

### New Features

- X11 now supports IME (opt-in via `use_ime = true`). Thanks to @H-M-H.
- Colors 16-255 definable in `colors` and color scheme definitions. Thanks to @potamides.
- `SendKey`, `Multiple` key assignment actions.
- `use_resize_increments` option.
- `visual_bell`, `audible_bell` configuration and `bell` event.
- `wezterm.action_callback` function. Thanks to @bew.
- `window_padding` as cell/percentage values.
- SSH client now uses `libssh` by default; `ssh_backend` config option.
- `unicode_version` option and corresponding OSC escape sequences.
- `canonicalize_pasted_newlines` option.
- `wezterm.background_child_process` function.
- `unzoom_on_switch_pane` option. Thanks to @yyogo.
- `mux_env_remove` setting.
- macOS: binaries now codesigned.

### Bug Fixes

- 30+ fixes including `text_background_opacity`, cursor transparency, X11 laggy input, Wayland HiDPI, font matching in TTC files, multiplexer session tracking, and more.

---

## 20210814-124438-54e29167

**Date**: 2021-08-14 | **Tag**: [`20210814-124438-54e29167`](https://github.com/Dicklesworthstone/wezterm/commit/54e29167ba25dd4b51bb4ceccfe92941b98d94e1)

### New

- `window_background_gradient` with radial gradient support.
- Kitty image protocol: basic animation, source/offset options, deflate support.
- `add_to_config_reload_watch_list` for watching additional config files.
- Window state concept (fullscreen, maximized).
- Bold/dim/italics synthesis for fonts.
- `CTRL-SHIFT-L` assigned to `ShowDebugOverlay`.
- Sextant and braille glyphs as custom block glyphs.
- `bypass_mouse_reporting_modifiers` option.
- `window_frame` for Wayland window decorations.
- `window:get_appearance()` for dark mode detection.
- `wezterm cli spawn --new-window`.
- `wezterm ls-fonts --list-system` and `--text` output.
- `COLORTERM=truecolor` now set in environment.

### Bug Fixes

- SSH config path fixed (`/etc/ssh/ssh_config`), `Include` statements processed.
- UTF8-encoded-C1 control codes, double-click-drag selection panic, Sixel HLS hue handling, split separator positioning, OSC 7 paths with spaces, and many more.

---

## 20210502-154244-3f7122cb

**Date**: 2021-05-02 | **Tag**: [`20210502-154244-3f7122cb`](https://github.com/Dicklesworthstone/wezterm/commit/3f7122cb3f9d2fd92fb836e2f4b2aa7b839b6c86)

Lua config system improvements. `wezterm.config_dir` and `wezterm.config_file` variables.

Prior candidate: `20210502-130208-bff6815d`.

---

## 20210405-110924-a5bb5be8

**Date**: 2021-04-05 | **Tag**: [`20210405-110924-a5bb5be8`](https://github.com/Dicklesworthstone/wezterm/commit/a5bb5be80aa5e9e5ae49400507a43523ae2edf2d)

Font system fixes for cross-platform compatibility.

Prior candidates: `20210404-111518-fe48951e`, `20210404-112810-b63a949d`.

---

## 20210314-114017-04b7cedd

**Date**: 2021-03-14 | **Tag**: [`20210314-114017-04b7cedd`](https://github.com/Dicklesworthstone/wezterm/commit/04b7cedd02eb533fa9eb3c3278764e982044ff96)

- Fixed animated GIF playback.
- Full alpha blending for glyphs.

---

## 20210203-095643-70a364eb

**Date**: 2021-02-03 | **Tag**: [`20210203-095643-70a364eb`](https://github.com/Dicklesworthstone/wezterm/commit/70a364eb3fd5624d950f6ed87a239f97b00c6028)

### New

- Curly, dotted, dashed, and colored underline support.
- Overline support.
- `bold_brightens_ansi_colors` option.
- `tab_max_width` option.
- `adjust_window_size_when_changing_font_size` option.
- DECRQSS, DECRPTUI support.
- CBT (cursor backward tabulation) implementation.
- Non-24bpp display support improvements.

### Fixed

- Scroll wheel to cursor mapping in alt screen.
- Texture space errors at large font sizes.
- X11 painting issues.
- Windows initial window size with display scaling != 100%.

---

## 20201101-103216-403d002d

**Date**: 2020-11-01 | **Tag**: [`20201101-103216-403d002d`](https://github.com/Dicklesworthstone/wezterm/commit/403d002d0a81d264b00611ee8f3f7591ad41b492)

- Fixed potential crash with multiple EGL windows on Windows.
- Fixed window resize behavior on Windows.

Prior candidate: `20201031-154415-9614e117`.

---

## 20200909-002054-4c9af461

**Date**: 2020-09-09 | **Tag**: [`20200909-002054-4c9af461`](https://github.com/Dicklesworthstone/wezterm/commit/4c9af4617961e8cad8ca682708f1dba08221d63c)

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

- F5+ key representations corrected.
- Hyperlink matching improvements for double-wide characters.
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

- MS Terminal mode 25 cursor visibility support.

---

## 20200202-181957-765184e5

**Date**: 2020-02-02 | **Tag**: [`20200202-181957-765184e5`](https://github.com/Dicklesworthstone/wezterm/commit/765184e507f9959bb7f92b713b0f01db00ebbd4d)

Multiple build candidates on the same day (`2489abf9`, `765184e5`, `b07ed454`). Included fixes for Windows tag builder and multi-line 2FA prompts.

---

## 20200113-214446-bb6251f

**Date**: 2020-01-13 | **Tag**: [`20200113-214446-bb6251f`](https://github.com/Dicklesworthstone/wezterm/commit/bb6251fad9e4baf658d7edb1a88fd7708e0acd6e)

- Fixed CLI proxy statistics printing interference.

Prior candidate: `20200113-222147-724ad3a`.

---

## 20191218-101156-bf35707

**Date**: 2019-12-18 | **Tag**: [`20191218-101156-bf35707`](https://github.com/Dicklesworthstone/wezterm/commit/bf35707cec4f70538980b49602c2972c5fbbd3a9)

Documentation theme switched to "Hacker" (darker).

---

## 20191124-233250-cb9fd7d

**Date**: 2019-11-24 | **Tag**: [`20191124-233250-cb9fd7d`](https://github.com/Dicklesworthstone/wezterm/commit/cb9fd7db5b56d13ba782f3fd521ec19f1ce997b2)

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
| `20190324-175217-2667d6a` | 2019-03-24 | [`2667d6a`](https://github.com/Dicklesworthstone/wezterm/commit/2667d6a5edf6dce4111ecc80a43fdfdf7bdc74ff) | Appveyor deploy patterns |
| `20190324-160658-560087c` | 2019-03-24 | [`560087c`](https://github.com/Dicklesworthstone/wezterm/commit/560087c5e7784207a701ba03082329772e841656) | Earliest tagged build |

The project was started on 2017-12-07 with the [initial commit](https://github.com/Dicklesworthstone/wezterm/commit/c53ca64c33d1658602b9a3aaa412eca9c6544294) "kick things off with a readme".

---

## Crate-Level Tags

These tags represent published versions of WezTerm's library crates on [crates.io](https://crates.io),
independent of the main application release cycle.

### termwiz (Terminal Widgets Library)

| Tag | Date | Notes |
|-----|------|-------|
| `termwiz-0.23.3` | 2025-03-20 | Latest; includes `wezterm-escape-parser` extraction, `no_std` support |
| `termwiz-0.23.2` | 2025-03-19 | Bugfixes: tmux -CC `%config-error`, Windows Terminal mouse SGR |
| `termwiz-0.23.1` | 2025-03-19 | Bugfixes |
| `termwiz-0.23.0` | 2025-02-10 | Spawnable LineEditor, ConEmu progress OSC, optional tmux/image features |
| `termwiz-0.22.0` | 2024-01-27 | Concurrent with 20240127 release cycle; bitflags 2.0, F13-F24 support |
| `termwiz-0.20.0` | 2023-02-12 | Horizontal scroll support, performance improvements |
| `termwiz-0.19.0` | 2022-11-02 | Unicode 15, `modifyOtherKeys`, 32-bit fixes |
| `termwiz-0.18.0` | 2022-09-22 | Clustered line storage, reduced per-line memory, nerdfont symbol updates |
| `termwiz-0.17.1` | 2022-08-03 | winapi build fix |
| `termwiz-0.17.0` | 2022-08-02 | Kitty image protocol, APC sequences, 10bpc color, bidi sequences, clustered line storage |
| `termwiz-0.16.0` | -- | Concurrent with 20220408 release |
| `termwiz-0.13.0` | 2021-04-14 | Sixel support, overline, leader keys, Cell memory optimization |
| `termwiz-0.12.0` | -- | Concurrent with 20210404 release |
| `termwiz-0.9.0` | 2020-05-17 | Windows console improvements, line editor enhancements |
| `termwiz-0.8.0` | -- | Concurrent with 20200406 release |
| `termwiz-0.7.0` / `0.7.1` | 2020-02/04 | Removed palette dependency, poll fixes |
| `termwiz-0.6.0` | 2020-01-18 | Cursor style restore, input parser fixes |
| `termwiz-0.5.0` | 2019-12-22 | Configuration trait extraction |
| `termwiz-0.4.0` | 2019-06-30 | Version update |
| `termwiz-0.3.0` / `0.3.1` | 2019-06-02/03 | filedescriptor crate adoption, spurious event fixes |
| `termwiz-0.2.0` | -- | Early 2019 |
| `termwiz-0.1.0` | 2019-05-28 | Initial release with line editor, tab completion, history |

Note: Tags `termiz-0.23.1`, `termiz-0.23.2`, `termiz-0.23.3` are typo variants of the corresponding `termwiz-*` tags.

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

*This changelog was rebuilt on 2026-03-21 from exhaustive research of git log, git tags, and the upstream docs/changelog.md, spanning the project's history from the initial commit on 2017-12-07 through 8,600+ commits to present. It covers 33 dated application releases, 4 nightly tags, and 17 crate-level version tags. All commit links point to the [Dicklesworthstone/wezterm](https://github.com/Dicklesworthstone/wezterm) fork.*
