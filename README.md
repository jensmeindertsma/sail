<div align="center">
    <h1>⛵ sail</h1>
    <i>Seamless self-owned application deployment.</i>
</div>
<br/>

## Introduction

## Development

```bash
$ sudo apt-get update
$ sudo apt-get install ca-certificates curl
$ sudo install -m 0755 -d /etc/apt/keyrings
$ sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
$ sudo chmod a+r /etc/apt/keyrings/docker.asc

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

$ sudo apt-get update
```

```bash
$ sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

```bash
$ sudo usermod -aG docker <username>
$ newgrp docker
$ docker info
$ $ocker run hello-world
Unable to find image 'hello-world:latest' locally
latest: Pulling from library/hello-world
17eec7bbc9d7: Pull complete
Digest: sha256:56433a6be3fda188089fb548eae3d91df3ed0d6589f7c2656121b911198df065
Status: Downloaded newer image for hello-world:latest

Hello from Docker!
This message shows that your installation appears to be working correctly.

To generate this message, Docker took the following steps:
 1. The Docker client contacted the Docker daemon.
 2. The Docker daemon pulled the "hello-world" image from the Docker Hub.
    (amd64)
 3. The Docker daemon created a new container from that image which runs the
    executable that produces the output you are currently reading.
 4. The Docker daemon streamed that output to the Docker client, which sent it
    to your terminal.

To try something more ambitious, you can run an Ubuntu container with:
 $ docker run -it ubuntu bash

Share images, automate workflows, and more with a free Docker ID:
 https://hub.docker.com/

For more examples and ideas, visit:
 https://docs.docker.com/get-started/
```

1. Nix

```bash
$ curl -fsSL https://install.determinate.systems/nix | sh -s -- install --determinate`
```

2.

```bash
$ nix profile add 'nixpkgs#cowsay'
$ cowsay hi
 ____
< hi >
 ----
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

3

```toml
$ cat ~/.config/direnv/direnv.toml
[global]
hide_env_diff = true
warn_timeout = 0
```

4. Clone the repository

```
$ git clone git@github.com:jensmeindertsma/sail.git
```

Inside the directory run

```
$ nix profile add nixpkgs#direnv
$ echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
$ direnv allow .envrc
```

Direnv extension is critical otherwise rust-analyzer won't work

you'll need to reboot the machine and fully disconnect and quit VSCode for direnv to work.

- ```
  $ mkdir ~/.config
  ```
- Then copy over your config

  ```
  $ scp -r ~/.config/git marina:~/.config/
  ```

- Installing build files
  - Running the `just` script:
    ```
    $ just install
    ```

Now you are in the sail group, but reload your shell session to be able to run `sail` without sudo.
