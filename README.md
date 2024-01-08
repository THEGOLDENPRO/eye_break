<div align="center">

  # 👀 eye break
  
  <sub>Little program I made in 🦀Rust that reminds me every 20 minutes to look away from my computer 🖥screen.<sub>
  
  <img width="320px" src="https://user-images.githubusercontent.com/66202304/230767901-a282d2fc-3dc4-43ee-a4c3-7573e55a344e.png">
  <img width="320px" src="https://user-images.githubusercontent.com/66202304/230767905-75bebf9e-a015-4388-9d44-d0344336f57f.png">

</div>

## Installation 🛠️
Install from source (like a real man).

Prerequisites: **[``git``](https://git-scm.com/downloads), [``rust-lang``](https://www.rust-lang.org/tools/install), [``make``](https://www.gnu.org/software/make/) (recommended), [``systemd``](https://systemd.io/)**

### Linux 🐧
```sh
git clone https://github.com/THEGOLDENPRO/eye-break
cd eye-break
```
Now if you have 'make' you may just run these commands:
```sh
make # build
make install # install to bin, install assets and add service to systemd
```
> If you don't have 'make' for some reason go and copy the code from the [makefile](https://github.com/THEGOLDENPRO/eye_break/blob/master/Makefile) yourself but try the make command at least you might already have it and I highly recommend you install it.

Enable the systemd service.
```sh
systemctl --user enable eye-break
```
Start it!
```sh
systemctl --user start eye-break
```
**then done 🥳**

### Windows 🪟
*It's very complicated...* but I'll probably get at it one day.

<br>

> I stay way too long on the computer without taking eye breaks so I was like hey here's my second excuse to code something in Rust and that's how we ended up here. 😐 This script follows the **[20:20:20 rule](https://www.google.com/search?q=20%3A20%3A20+rule)**.