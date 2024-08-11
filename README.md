# sail

Application deployment that is easy, reliable and self-owned.

## TODO

- [ ] proxying logic
- [ ] persistent configuration,
- [ ] Implement upload endpoint for Docker images
  - Allow secret key creation during app creation that can be rotated with the CLI. This secret must be provided when uploading an app as a security/protection feature.
- [ ] Set up Docker image for `rooster`
- [ ] Set up CI for `rooster` that uploads to the endpoint
- [ ] Implement Docker container restarting when new images are uploaded
- [ ] Implement SQLite database volume attachment and providing database url
- [ ] implement database backup on deploy
- [ ] Implement more CLI commands that give an overview of current status
