# sail

Seamless self-owned application deployment.

## Tasks
- [ ] Add `daemon` crate that:
  - [x] implements logging with `tracing` and `tracing-subscriber`
  - [x] logs startup
  - [ ]logs uptime every 10 seconds
  - [x] waits for shutdown signal (`SIGTERM`) from `systemd` and logs the shutdown
- [x] Set up scripts for installing, updating, and uninstalling the Sail program
  - [x]`enable --now` or stop `systemd` service and socket
  - [x] add or remove `systemd` files and reload `systemd`
  - [x] add, update, or remove binaries from `/usr/local/bin`
  - [x] watch daemon logs during development with a `just watch` command
- [ ] Write `CONTRIBUTING.md` guide
- [x] Implement basic message passing between CLI and daemon over `systemd` socket
- [ ] Add HTTP listener on `localhost` port `1312` (make it serve basic HTML page with hostname on it)
  - Install Nginx and provide several configuration templates inside `docs` directory
    - Transparent (listen on port 80 only) and proxy traffic for any hostname directly to Sail
    - Upgrading (listen on port 80 and 443) and redirect all HTTP traffic to HTTPS, proxy HTTPS traffic to Sail
      - Document how to set up Let's Encrypt `certbot` or how to get SSL certificate from DNS like Cloudflare (see [Custom SSL/TLS](https://developers.cloudflare.com/ssl/origin-configuration/ssl-modes/#custom-ssltls))
    - Strict SSL (listen on port 443 only) and use Let's Encrypt
    - Cloudflare Strict SSL with [Authenticated Origin Pulls](https://developers.cloudflare.com/ssl/origin-configuration/authenticated-origin-pull/set-up/zone-level/)
