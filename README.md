# spotdj
A quick and dirty application to leverage Spotify DJ's TTS API. In specific, this provides a Discord bot that can be utilized with slash commands.

As of writing, this currently uses a fork of [librespot-org/librespot](http://github.com/librespot-org/librespot) underneath [spotlightishere/librespot](https://github.com/spotlightishere/librespot) to correctly leverage Protobuf-related APIs.

## Usage
1. Set the environment variables `SPOTIFY_USERNAME` and `SPOTIFY_PASSWORD` to the username and password of your premium Spotify account.
   1. Free accounts are not supported by librespot, and subsequently `spotdj` cannot support free accounts.
   2. If you sign in to Spotify via Facebook, Apple, etc., you can still reset your account's password. This will not break third-party login.
2. Set the environment variable `DISCORD_TOKEN` to the token of a bot you've created via [Discord's Developer Portal](https://discord.com/developers/applications).
3. `cargo run`, and enjoy!

<!-- Example video of bot usage -->
https://github.com/spotlightishere/spotdj/assets/10055256/84968d2e-1ea4-4cdd-ad4a-84a3c74f1b15
