play-binding:
    cargo run --release -- \
      -p schip \
      --load-increment-i-with-x-quirk \
      --jump-using-x-quirk \
      --shift-ignore-vy-quirk \
      --sleep 15 \
      --set-disabled-color 0xF2A261 \
      --set-first-plane-color 0xEB6C05 \
      roms/binding.ch8

play-superneatboy:
    cargo run --release -- \
      -p xochip \
      --load-increment-i-with-x-quirk \
      --wrap-instead-of-clipping-quirk \
      --scale 11 \
      --set-disabled-color 0x020C12 \
      --set-second-plane-color 0xF70C8A \
      --set-first-plane-color 0xDCF0FC \
      --set-both-plane-color 0xDCF0FC \
      roms/superneatboy.ch8

play-skyward:
    cargo run --release -- \
      -p xochip \
      --sleep 10 \
      --scale 7 \
      --wrap-instead-of-clipping-quirk \
      --set-disabled-color 0x020C12 \
      --set-first-plane-color 0xDCF0FC \
      --set-second-plane-color 0xE31B3D \
      --set-both-plane-color 0xE31B3D \
      roms/skyward.ch8
