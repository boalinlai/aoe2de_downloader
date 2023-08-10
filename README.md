# aoe2de_downloader

script for Age of Empire DE record file in Rust to help download the record file for game review

# How to use

- copy and paste the downloader binary into saved game folder,
On windows, it should prompt the followings:

```
Please enter a number for download_leaderboard_top_n:
5
"[aM] Hera": (rating:2910, rank:0)
"[aM] Hearttt": (rating:2751, rank:1)
"_Barles_": (rating:2724, rank:2)
"___MbL___": (rating:2710, rank:3)
"SalzZ_Vinchester": (rating:2707, rank:4)
download time for [aM] Hearttt: 2.74s
download time for _Barles_: 2.70s
```

# Motivation

# build under linux

- for building binary for windows

```
cargo build --release --target x86_64-pc-windows-gnu
```

for debugging under linux env

```
cargo build
```
