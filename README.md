# osu!recoverer
Downloads every map you have ever played in [osu!](https://osu.ppy.sh/) (if you were logged in when you played them)

# Get Started
## Installation
To get started either download the latest release [clicking here](https://github.com/NeutronX-dev/osu-recoverer/releases/latest/download/osu-recoverer.exe) or build the code yourselg with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```sh
git clone https://github.com/NeutronX-dev/osu-recoverer
cd ./osu-recoverer
cargo build --release
./target/release/osu-recoverer.exe
```
## Usage
When you open the file you will be prompted with your osu! ID like this
```
osu!user_id: _
```
- Go to your profile and take a look at the URL.
- - Should look like this: <a href="https://osu.ppy.sh/users/22090473">https://osu.ppy.sh/users/<u><strong>22090473</strong></u></a>
- Copy that number and paste it into osu!recoverer

Then it will ask you for a session
```
osu!user_id: <your_osu_id>
osu!session: _
```
here you need to put a cookie you find in your cookies.
* Go to your osu! profile
* Press **CTRL + Shift + I** and go to the **Network** Tab
* Refresh the current site.
* Scroll all the way up and click on the first item (should be named as your user ID)
* On the new panel to the left click on **Cookies**
* Double click the **Value** of the `osu_session` cookie, and copy it.
* Paste into osu!recoverer
# What now?
1. It will start fetching all the maps which can take up to a few minutes depending on how many plays you have. (Go to [`osu.rs:6`](./src/osu.rs#L6))
2. It will remove duplicate beatmapsets. (only downloads one map if you played 2 difficulties of a single beatmapset) (Go to [`main.rs:33`](./src/main.rs#L33))
3. It will download the beatmaps and save them into a new folder named `maps`.