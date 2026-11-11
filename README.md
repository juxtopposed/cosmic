# Pixelated Dynamic Blue Theme

This is a collection of the relevant files for my COSMIC's theme based on [this video](https://youtu.be/57j7W7coQMg).

## Instructions

### Colors

1. Download `Dark.ron` or `Light.ron`
2. Go to COSMIC Settings > Desktop > Appearance > Import
3. The correct theme will automatically be applied to dark or light mode

### Fonts

1. Download the Monocraft Nerd Fonts Patched file from [this page](https://github.com/IdreesInc/Monocraft/releases/)
2. Go to COSMIC Settings > Desktop > Appearance > System Font or Monospace Font > Select Monocraft
3. In GNOME Tweaks App (You can get it from the COSMIC Store), also select Monocraft in the Font tab.

### Custom Cursor

1. Download `JuxCursor.zip`
2. Extract it in `~/.local/share/icons/`
3. Edit `/etc/environment` and add this to it
```
XCURSOR_SIZE=32
XCURSOR_THEME=JuxCursor
```
You can change the number `32` to `16` if you want the cursors to be smaller.

4. Edit `/etc/alternatives/x-cursor-theme` to look like this:
```
[Icon Theme]
Inherits=JuxCursor,Adwaita
```
5. In the terminal, run this so GTK apps follow the cursor theme:
```
gsettings set org.gnome.desktop.interface cursor-theme JuxCursor
```
6. In the terminal, run this so Qt apps also follow the theme (make sure to replace "user" with your own username):
```
ln -s /usr/share/icons/default /home/user/.local/share/icons/default
```
7. Reboot

### Twilite (Dynamic Light/Dark Wallpaper Switcher)

1. Install `awww` from source because it is not available in the official Debian/Ubuntu repositories (including Pop!_OS):
```
git clone https://codeberg.org/LGFae/awww.git
cd awww
cargo build --release
sudo install -m 755 target/release/awww /usr/local/bin/awww
sudo install -m 755 target/release/awww-daemon /usr/local/bin/awww-daemon
```
2. Download `twilite.zip` from this repo
3. Extract in Home folder
4. In the terminal, run this:
```
cd twilite
./install.sh
```
It will automatically install the dependencies.

5. Edit the config file at `~/.config/twilite/config.toml` and paste the correct paths to your wallpapers. Example:
```toml
# Valid transition types for `awww img`:
#   simple, left, right, top, bottom, center, outer, any, random
# Valid transition_step and transition_fps values: 1..255
light = "/path/to/light.png"
dark = "/path/to/dark.png"
transition_type = "simple"
transition_step = 2
transition_fps = 30
```
6. The app works based on your system theme. So, whenever you turn on your PC, launch the app, and it will automatically set your specified wallpaper for the current theme.


### COSMIC Terminal Theme

1. Download `JuxTermDark.ron` and `JuxTermLight.ron`
2. Open COSMIC Terminal and go to View > Color Schemes
3. Import each theme into the correct Light/Dark tab
4. Go to View > Settings and apply the correct theme to each mode

### Icons (Work in Progress)

Note: This pack is currently being updated. It is INCOMPLETE. There are A LOT of icons left, so bear with me while I work on it. If you'd like to help in the design to complete the pack faster, just let me know in the Issues tab which category you want to work on. I will add your name to the credits as a thank you!

Currently, the icon pack is mashup of [YAMIS for COSMIC](https://github.com/harpoonwithaz/YAMIS-Cosmic) and my pixelated icons. The goal is to redesign all of them into the pixelated theme.


## Screenshots

<img width="1920" height="1080" alt="light" src="https://github.com/user-attachments/assets/785dcaa1-13d0-466d-aae1-5ad152d553b2" />

<img width="1920" height="1080" alt="dark" src="https://github.com/user-attachments/assets/e7513e78-e7a9-4c15-8efa-bb2a91a19c93" />
