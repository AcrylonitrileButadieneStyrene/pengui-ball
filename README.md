# :penguiBall:

![Static Badge](https://img.shields.io/badge/artificial%20intelligence-forbidden-red)
![GitHub contributors](https://img.shields.io/github/contributors/AcrylonitrileButadieneStyrene/pengui-ball)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/AcrylonitrileButadieneStyrene/pengui-ball)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/AcrylonitrileButadieneStyrene)](#contributing)
[![Static Badge](https://img.shields.io/badge/license-MIT%20or%20Apache%202.0%20with%20exceptions-green)](#license)

Alternative frontend client for [YNOproject](https://ynoproject.net/).

A [public instance](https://pengui-ball.jackssrt.com/) 
is being hosted by 
[@AcrylonitrileButadieneStyrene](https://github.com/AcrylonitrileButadieneStyrene).

${\color{transparent}█████\color{#4646e8}█\color{#7070ee}███\color{#4646e8}█\color{transparent}█████}$<br>
${\color{transparent}███\color{#7070ee}█\color{#4646e8}█\color{white}█\color{#4646e8}█\color{#7070ee}█\color{#4646e8}█\color{white}█\color{#4646e8}█\color{#7070ee}█\color{transparent}███}$<br>
${\color{transparent}██\color{#7070ee}███\color{#4646e8}█\color{#eeee70}███\color{#4646e8}█\color{#7070ee}███\color{transparent}██}$<br>
${\color{transparent}█\color{#7070ee}██\color{#dec98b}███\color{#eec370}███\color{#dec98b}███\color{#7070ee}██\color{transparent}█}$<br>
${\color{transparent}█\color{#7070ee}█\color{#dec98b}███\color{#ffffd6}█████\color{#dec98b}███\color{#7070ee}█\color{transparent}█}$<br>
${\color{#dec98b}████\color{#ffffd6}█\color{#1e1e1e}██\color{#ffffd6}█\color{#1e1e1e}██\color{#ffffd6}█\color{#dec98b}████}$<br>
${\color{#dec98b}████\color{#ffffd6}███████\color{#dec98b}████}$<br>
${\color{#dec98b}████\color{#ffffd6}███████\color{#dec98b}████}$<br>
${\color{#7070ee}████████\color{#7070ee}███████}$<br>
${\color{#7070ee}██████\color{white}███\color{#7070ee}██████}$<br>
${\color{transparent}█\color{#7070ee}████\color{white}█████\color{#7070ee}████\color{transparent}█}$<br>
${\color{transparent}█\color{#7070ee}████\color{white}█████\color{#7070ee}████\color{transparent}█}$<br>
${\color{transparent}██\color{#7070ee}█\color{#eeee70}███\color{white}███\color{#eeee70}███\color{#7070ee}█\color{transparent}██}$<br>
${\color{transparent}███\color{#eeee70}████\color{white}█\color{#eeee70}████\color{transparent}███}$<br>
${\color{transparent}█████\color{#eeee70}██\color{white}█\color{#eeee70}██\color{transparent}█████}$<br>

# Setup
- (Optional) `cargo install `[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)
  - If you choose to not use this, replace `binstall` with `install` for the rest of the commands.
- `cargo binstall `[`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos)
- (Optional) `cargo binstall `[`just`](https://github.com/casey/just)
- (Optional) `cargo binstall `[`stylance-cli`](https://github.com/basro/stylance-rs)
- (Optional) `cargo binstall `[`leptosfmt`](https://github.com/bram209/leptosfmt)
- Install [nginx](https://nginx.org/):
  - Windows: [`scoop`](https://scoop.sh)` install nginx`
  - Arch: `pacman -S nginx`
  - Ubuntu/Debian/Mint: 
    use a [real OS](https://www.google.com/search?q=literally+anything+else)

# Usage
1. Run the server binary (via e.g. `cargo leptos serve`)
2. Run `just serve`
3. Open https://localhost:8080/

## Developing
- `cargo leptos watch` (Recompile on change)
- `just css` (Recompile CSS on change)
- `just serve` (Run nginx on port 8080)

## Formatting
- `cargo fmt` (Outside of views)
- `just fmt` (Inside of views)

# Contributing

You can contribute by spreading the good word of pengui-ball to new users :fluoPray:

If you make a pull request, notify me somehow or it will never be seen.

# License
This project is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), 
at your choice, with exceptions listed in [LICENSE](LICENSE).
