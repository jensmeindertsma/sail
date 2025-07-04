# ⛵ sail

Seamless self-owned application deployment.

## Introduction

Sail is a tool born out of the scripts I wrote to make deploying multiple application containers on a single VPS. Typically this involves tools like Docker, Nginx, and SQLite. My main script `deploy.sh` takes care of stopping the existing container, starting up the new container, handling environment variables and persistent volumes to save the database across deployments.

However, I wanted a nicer way of managing this deployment infrastructure and gives me a better overview. This led me to create Sail: a command line tool that simplifies managing Docker and Nginx to help you deploy your applications with ease.

Sail is currently <ins>**still in development**</ins>!
