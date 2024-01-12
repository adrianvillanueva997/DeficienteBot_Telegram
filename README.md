![Build Docker image](https://github.com/adrianvillanueva997/DeficienteBot_Telegram/workflows/Build%20Docker%20image/badge.svg)

# DeficienteBot_Telegram

Private telegram bot made in rust that calls idiot to someone who sends a message with certain content to a group chat. Also it sends copypastas when certain words are detected and it converts webm videos to mp4.

# Deployment

Create a webhook using the software of your choice, for local development i use ngrok. Export the telegram key in an environment variable like this:

```bash
export TELOXIDE_TOKEN=yourtoken
export url=yoururl
```

Then run the bot with cargo:

```bash
cargo run
```

In the docker-compose file you have my personal configuration, you can use it as a guide to deploy it in your own server. I am using a reverse proxy to redirect the traffic to the bot, you can use nginx or apache for this. I am using traefik.

# Toolset

- [Rust](https://www.rust-lang.org/)
- [ffmpeg](https://ffmpeg.org/)
- [Docker](https://www.docker.com/) or [Podman](https://podman.io/)
- [Traefik](https://traefik.io/)
- [Telegram](https://telegram.org/)
- [ngrok](https://ngrok.com/)

# License

This project is licensed under the GNU GPL3 License - see the [LICENSE.md](LICENSE.md) file for details
