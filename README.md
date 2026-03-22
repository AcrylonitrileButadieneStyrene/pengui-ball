# :penguiBall:
Alternative frontend client for [YNOproject](https://ynoproject.net/).

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

# License
This project is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), 
at your choice, with exceptions listed in [LICENSE](LICENSE).
