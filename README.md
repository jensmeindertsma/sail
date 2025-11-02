<div align="center">
    <h1>⛵ sail</h1>
    <i>Seamless self-owned application deployment.</i>
</div>
<br/>

## Introduction

I needed a robust software program that can run on a VPS and host multiple applications packaged as Docker images, and provide means for each of these applications to have a database.

You will be able to push (under authentication) Docker images to a public endpoint (i.e. `sail.<yourdomain>.com`), and this would trigger a re-deployment of your application, and with minimal downtime your new version will be live to the world.

Internally, Sail manages Nginx and Docker for you, so you can focus on shipping your code. Sail comes bundled with a CLI interface which can be used by SSH'ing into the VPS and allows you to create, edit, and delete your app configurations.

## Development

Because Sail is not production ready, I'll cover the steps to getting a development environment together. To work on Sail you'll need a Linux virtual machine. The `saild` daemon runs under `systemd`, and so developing in a virtual machine is the only way to realistically test the implementation. Let's go over the installation of the required tools on the VM:

### Docker

Docker is what will be running our applications. The first commands add the Docker download source to APT.

```bash
$ sudo apt-get update

$ sudo apt-get install ca-certificates curl

$ sudo install -m 0755 -d /etc/apt/keyrings

$ sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc

$ sudo chmod a+r /etc/apt/keyrings/docker.asc

$ echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

$ sudo apt-get update
```

This allows us to install the Docker packages.

```bash
$ sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Now we can verify that Docker is succesfully running:

```bash
$ sudo usermod -aG docker <username>

# Needed to make the new group partnership effective until a full shell reset.
$ newgrp docker

$ docker info

$ docker run hello-world

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

### Nginx

Nginx is what will be proxying the web request coming into the server to the right Docker container. Installing Nginx is really simple:

```bash
$ sudo apt install nginx -y

# Check that Nginx is running.
$ nginx -v
```

### Nix

Nix is a package manager that can be used to create fully-reproducible development environment. It packages all the dependencies together so everyone works with the same.

```bash
$ curl -fsSL https://install.determinate.systems/nix | sh -s -- install --determinate`
```

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

We need `direnv` to enter the development environment with it's packages when we enter the directory:

```bash
$ nix profile add nixpkgs#direnv

$ echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
```

I configured some settings to make the output of `direnv` less verbose.

```bash
$ cat ~/.config/direnv/direnv.toml

[global]
hide_env_diff = true
warn_timeout = 0
```

Now it is time to clone the repository onto the VM.

```bash
$ git clone git@github.com:jensmeindertsma/sail.git
```

Now, to open up the development environment, run:

```bash
$ direnv allow .envrc
```

This will start fetching packages to put together the environment.

> When you are using Visual Studio Code with the Remote SSH extension you should install the `mkhl.direnv` extensions. When installed it's critical to reboot the machine, disconnect VSCode, then close VSCode altogether, then reconnect. **This extension is required to make rust-analyzer work**.

To make commits and push to GitHub, you'll need your SSH key (I recommend `ForwardAgent yes` inside your host `.ssh/config`) and then clone your host Git config over to the VM:

```bash
$ scp -r ~/.config/git marina:~/.config/
```

Now we are pretty much done. You can now build and install the current version of Sail, then get working on some changes:

```bash
$ just install
```

This script puts you in the `sail` group, but reload your shell session to be able to run `sail` without sudo.
