`# lemoa

Native Gtk client for Lemmy (beta)

## Installation

| Platform | Command                                                                                                                                                 |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Flathub  | <a href="https://flathub.org/apps/details/io.github.lemmygtk.lemoa"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |

If you don't use any of these platforms, consider [building the app from source](#Building).

# Screenshots
![](https://raw.githubusercontent.com/lemmygtk/lemoa/main/data/screenshots/community.png)
![](https://raw.githubusercontent.com/lemmygtk/lemoa/main/data/screenshots/posts.png)
![](https://raw.githubusercontent.com/lemmygtk/lemoa/main/data/screenshots/user.png)

# Features

- Listing trending posts
- Viewing a post and its comments
- Viewing profiles
- Viewing trending communities
- Searching communities
- Logging in with an account
- Writing posts or comments
- Viewing subscribed posts
- Following communities
- Voting for or against posts or comments
- Editing and deleting posts or comments
- Viewing the personal inbox (mentions, replies)
- Private messages

# Troubleshooting

### Flatpak installation doesn't follow the system Gtk theme

In order to apply a different theme when using Flatpak, run
```
flatpak override --filesystem=/usr/share/themes/
flatpak override --env GTK_THEME=Adwaita-dark
```
You can replace `Adwaita-dark` with the name of any other Gtk theme you have installed.

If you don't use Flatpak, the correct Gtk Theme should be applied automatically.

# Build dependencies

- rust
- cargo
- pkg-config
- libgtk-4-dev or gtk4-devel (name depends on the distro)

# Building

## Building with meson

```
meson _build
ninja -C _build
sudo ninja -C _build install
```

## Building with Docker

```
sudo docker build --no-cache . -t lemoa:latest
CONTAINER_ID=$(sudo docker create --name lemoa -t lemoa:latest)
sudo docker cp $(CONTAINER_ID):/root/lemoa/target/release/lemoa .
```

Once the build is done, there will be an executable `lemoa` binary file in your current directory, executing it starts Lemoa :tada:.

## Building the binary only

Not recommended: To only install the binary (can only be started with the terminal), run

```sh
cargo install --git https://github.com/lemmygtk/lemoa.git
```
s
You can then start the app via the terminal after adding cargo's bin directory to your PATH variable.

# Development

Clone the repository and run

```sh
cargo run
```

# Credits
- [Original Icon](https://game-icons.net/1x1/delapouite/rat.html) by [Delapouite](https://delapouite.com/) under [CC BY 3.0](http://creativecommons.org/licenses/by/3.0/)
- Modified icon with gradient by [Ategon](https://programming.dev/u/Ategon)
- [Relm4](https://github.com/Relm4/Relm4) for providing such an awesome Gtk crate for Rust

# License

Lemoa is licensed under the [**GNU General Public License**](https://www.gnu.org/licenses/gpl.html): You can use, study and share it as you want.
shshoshots
