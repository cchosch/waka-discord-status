# waka-discord-status
[![wakatime](https://wakatime.com/badge/user/82cdae6a-ce3c-4063-8986-f4c5ae89b50d/project/d38c5a21-8400-4d4d-b069-711752eb4357.svg)](https://wakatime.com/badge/user/82cdae6a-ce3c-4063-8986-f4c5ae89b50d/project/d38c5a21-8400-4d4d-b069-711752eb4357)

This is a program written in rust (made to run as a [systemd service](https://en.wikipedia.org/wiki/Systemd)).
It automatically checks [WakaTime](https://wakatime.com), a tracker for time spent coding, and updates your
discord status to show time spent coding today.

## Development Environment
First, create a `.env` in the root and add your `WAKA_KEY` and `DISCORD_TOKEN`.

```.env
WAKA_KEY=waka_3f13a219-bc72-a8dd-0321-3a9bf0e1c7ff
DISCORD_TOKEN=MZ1yGvKTjE0rY0cV8i47CjAa.uRHQPq.Xb1Mk2nEhe-4iUcrGOuegj57zMC
```

Then, to start the development server, run `make`. This will suppress cargo warnings and automatically reload on save.

## Compile
To compile, simply run `make prod` and the build will be in `./target/release/waka_discord_status`.

