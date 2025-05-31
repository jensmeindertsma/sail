# sail

Seamless self-owned application deployment.

## Notes
- The installer assumes Docker is running in rootless mode.
- We want to support zero-downtime deploys by starting containers before the old one is stopped, sending HTTP requests at them until we get a response, then switch the proxy port to the new container after which we kill the old container. We need to figure out how to handle graceful shutdown for existing connections on the old container.

## Development Setup

1. [install Docker](https://docs.docker.com/engine/install/ubuntu/). Then, [switch Docker to rootless mode](https://docs.docker.com/engine/security/rootless/).

2. [Install Nix](https://nixos.org/download/#nix-install-linux)
3. Add `source /etc/profile.d/nix.sh` to `.profile`
4. Modify `~/.config/nix/nix.conf`:
   ```
   experimental-features = nix-command flakes
   max-jobs = auto
   ```
5. Install `direnv`:
   ```
   $ nix profile install nixpkgs#direnv
   $ echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
   ```
6. Make `direnv` quiet by setting `hide_env_diff` to `true` in `~/.config/direnv/direnv.toml`
7. Run `direnv allow`

You should now be able to run `rustc --version`:

```
$ rustc --version

rustc 1.89.0-nightly (d97326eab 2025-05-15)
```

Visual Studio Code should now prompt you to reload to apply the new environment to all the extensions. That is if you have installed the recommended extensions in `.vscode/extensions.json` (VSCode should prompt to install these).
