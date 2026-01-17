# :penguiBall:
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

# Building
## Setup
- `cargo install `[`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos)
- (Optional) `cargo install `[`just`](https://github.com/casey/just)
<!--
- (Optional) `cargo install `[`stylance-cli`](https://github.com/basro/stylance-rs)
-->
- (Optional) `cargo install `[`leptosfmt`](https://github.com/bram209/leptosfmt)
## Compiling
- `cargo leptos build --release`
## Developing
- `cargo leptos watch` (Recompile on change)
<!--
- `just css` (Recompile CSS on change)
-->
- `just serve` (Run nginx on port 8080)
## Formatting
- `cargo fmt` (Outside of views)
- `just fmt` (Inside of views)
