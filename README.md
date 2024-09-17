# sail

Application deployment that is easy, reliable and self-owned.

## Overview

- Config at /etc/sail/config.toml
- App config at /etc/sail/apps/foo.toml
- SQLite at /var?????
- Socket listener
- HTTP listener is of second importance, just make it respond with hello world to all
- Want less code inside select macro, functions better for rust-analyzer

- INSTALL SCRIPT, and self update ability!!
- sail update should load new binaries from github, replace them, then restart service.
-

- install script should fetch sail binary from github,

then run sail update
