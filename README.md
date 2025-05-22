# sail

Seamless self-owned application deployment.

The installer assumes Docker is running in rootless mode.

## User flow

1. User "creates" new app in cli by giving name and hostname(s).
   - App can have several hostnames
   - User is asked to set environment variables ahead of time
   - User is informed that there will be a "data" volume mounted
   - User is asked whether `DATABASE_URL` should be pre-set to a SQLite stored on the data volume.
2. User gets "secret" for authenticating to the Sail Docker endpoint.
3. The Docker endpoint accepts image pushes only with the right authentication
4. Docker then starts up this image as a container
   - Container is set to always restart automatically
5. All of this is logged so the user "pushing" the image from CI can see the progress.
6. Using the CLI, the current status can be retrieved.
   - Apps can be deleted
   - Apps can be backed up (only latest copy stored)
   - App's hostname may be modified.

## The web interface

Eventually I hope to implement a web interface where the traffic and stats of all of the apps can be visualized. I will not implement an admin view for each app's database, this is something the app can do themselves.

## Setting up for development

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
6. Run `direnv allow`

You should now be able to run `rustc --version`:

```
$ rustc --version

rustc 1.89.0-nightly (d97326eab 2025-05-15)
```

Visual Studio Code should now prompt you to reload to apply the new environment to all the extensions. That is if you have installed the recommended extensions in `.vscode/extensions.json` (VSCode should prompt to install these).
