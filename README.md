# sail

Seamless self-owned application deployment.

## Status

The server daemon and the CLI interface using systemd sockets has been implemented. The next steps include:
- A Docker Registry API implementation that allows authenticated image pushing
- Proxy implementation for delegating requests to managed Docker containers
- Web interface for managing settings & applications
