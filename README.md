# sail

Seamless self-owned application deployment.

## Introduction

Sail is a tool born out of the scripts I wrote to make deploying multiple application containers on a single VPS. Typically this involves tools like Docker, Nginx, and SQLite. My main script `deploy.sh` takes care of stopping the existing container, starting up the new container, handling environment variables and persistent volumes to save the database across deployments.

However, I wanted a nicer way of managing this deployment infrastructure. This led me to create Sail.

Currently I'm developing two separate versions:

- A CLI-only variant which interfaces directly with Docker and helps you set up the Nginx files that are needed
- A variant which also includes a "daemon" that manages HTTP requests directly and routes them to the correct container.
