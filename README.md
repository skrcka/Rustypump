# Rust syringe pump controller
Software designed for RPi 3B meant for controlling stepper driver via GPIO pins

## Links
- Backend - [https://github.com/skrcka/Rustypump](https://github.com/skrcka/Rustypump)
- Frontend - [https://github.com/skrcka/Reactpump](https://github.com/skrcka/Reactpump)
- Thesis - [https://www.overleaf.com/read/phstktcfyvdk](https://www.overleaf.com/read/phstktcfyvdk)

## Requirements
- Linux/WSL
- Docker installed
- RPi booted up and connected to internet
- rsync installed

## How to use
1. Go to my official docker image - ./goto-docker.sh
2. Build for RPi - ./build-rpi.sh or ./build-rpi-release.sh
3. Change IP and username in copy-to-rpi.sh or copy-to-rpi-release.sh
4. If sending to the device for the first time
    - Uncomment rsync for the ini file as well
    - Rsync service file - rsync -vP ./controller.service username@IP:/etc/systemd/system/
5. Copy to RPi - ./copy-to-rpi.sh or ./copy-to-rpi-release.sh
6. ssh to RPi
7. Enable it on RPi - sudo systemctl enable --now controller.service
