# ⛵ sail

Seamless self-owned application deployment.

## Introduction

Sail is a tool born out of the scripts I wrote to make deploying multiple application containers on a single VPS. Typically this involves tools like Docker, Nginx, and SQLite. My main script `deploy.sh` takes care of stopping the existing container, starting up the new container, handling environment variables and persistent volumes to save the database across deployments.

However, I wanted a nicer way of managing this deployment infrastructure and gives me a better overview. This led me to create Sail: a command line tool that simplifies managing Docker and Nginx to help you deploy your applications with ease.

Sail is currently <ins>**still in development**</ins>!

## Setting up the development environment

This project makes use of [Nix](<https://en.wikipedia.org/wiki/Nix_(package_manager)>). This allows for a declarative and reproducable environment. We highly recommend working on Sail inside a virtual machine, because Sail is a system-wide service that can impact your own operating system. We'll walk through how to set up a Sail development environment from scratch on a Ubuntu Server virtual machine.

0. We assume you have SSH access. Make sure you have set `ForwardAgent yes` in your `.ssh/config` file because you'll need the key to push to (your fork) of the repository. Run `ssh-add -l` to confirm that your key is accessible:

```bash
jens@marina:~$ ssh-add -l

256 SHA256:6F9DHp7reoug1NVCmiK5Vv2+CBskek1gqKzvufpK2C4 jens@vanguard (ED25519)
```

1. Installing Nix.

   - Nix is a declarative package manager that we use to provide a reproducible development environment.
   - We'll use a preconfigured Nix installation by running the install script:
     ```bash
     $ curl -fsSL https://install.determinate.systems/nix | sh -s -- install --determinate
     ```
   - Verify you are up and running:
     ```
     $ nix profile add nixpkgs#direnv
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

2. [Installing Docker](https://docs.docker.com/engine/install/ubuntu/)

   - Adding the `apt` repository:

     ```bash
     # Add Docker's official GPG key:
     $ sudo apt-get update
     $ sudo apt-get install ca-certificates curl
     $ sudo install -m 0755 -d /etc/apt/keyrings
     $ sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
     $ sudo chmod a+r /etc/apt/keyrings/docker.asc

     # Add the repository to Apt sources:
     $ echo \
     "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
     $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
     sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

     $ sudo apt-get update
     ```

   - Install the packages:
     ```bash
     $ sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
     ```
   - Add yourself to the `docker` group:
     ```bash
     $ sudo usermod -aG docker jens
     ```
   - Verify you can run containers:

     ```
     $ docker run hello-world
     Unable to find image 'hello-world:latest' locally
     latest: Pulling from library/hello-world
     c9c5fd25a1bd: Pull complete
     Digest: sha256:ec153840d1e635ac434fab5e377081f17e0e15afab27beb3f726c3265039cfff
     Status: Downloaded newer image for hello-world:latest

     Hello from Docker!
     This message shows that your installation appears to be working correctly.
     ```

3. Installing the repository:

   - ```
     $ git clone git@github.com:jensmeindertsma/sail.git
     ```
   - Open the remote repository in VSCode using the Remote SSH extension
   - Install the recommended extension when the window pops up in the bottom right corner (you may need to open a source file to receive the prompt)
   - Install `direnv`:
     ```
     $ nix profile add nixpkgs#direnv
     $ echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
     $ direnv allow .envrc
     ```
   - Give it some time! Nix is hard at work prepping your environment!
   - VSCode will reload

- Installing build files
  - Running the `just` script:
    ```
    $ just install
    ```
  - Now you should be able to run `sudo sail` command
  - You should add yourself to the `sail` group

4. Setting up Git

- ```
  $ mkdir ~/.config
  ```
- Then copy over your config
  ```
  $ scp -r ~/.config/git marina:~/.config/
  ```
