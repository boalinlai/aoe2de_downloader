# aoe2de_downloader

Script for Age of Empire DE record file in Rust to help download the record file for game review.

# Usage

- copy and paste the [windows downloader binary](https://github.com/boalinlai/aoe2de_downloader/blob/main/target/x86_64-pc-windows-gnu/release/aoe2de_downloader.exe) into saved game folder.

Under windows env, after double clicked exe, it will prompt the followings:

```
Please enter a number for download_leaderboard_top_n:
```

enter the number the app will start downloading record and unzip it under saved folder.

## Example output

```
Please enter a number for download_leaderboard_top_n:
10
"[aM] Hera": (rating:2910, rank:0)
"[aM] Hearttt": (rating:2762, rank:1)
"_Barles_": (rating:2724, rank:2)
"___MbL___": (rating:2710, rank:3)
"SalzZ_Vinchester": (rating:2707, rank:4)
"Gamdom.ACCM |AOEBuilds.com": (rating:2673, rank:5)
"Villese": (rating:2660, rank:6)
"[aM]MbL40C": (rating:2659, rank:7)
"Hera123": (rating:2646, rank:8)
"[aM] Nicov": (rating:2642, rank:9)
download time for [aM] Hearttt: 2.74s
download time for [aM] Nicov: 3.20s
download time for _Barles_: 2.08s
```

# Motivation

Downloading record files costs time,it required manually click download button, unzip, and moved the files under the saved folder. This cli utilized multi-thread to download all the selected files async for top N player.

# For Devs

## Build under linux

- for building binary for windows

```
cargo build --release --target x86_64-pc-windows-gnu
```

for debugging under linux env

```
cargo build
```
