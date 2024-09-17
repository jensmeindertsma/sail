# Setting up a VPS

Make sure to add yourself to the `sail` group so you can run the CLI without escalating privileges:

```sh
$ sudo usermod -aG sail {USERNAME}
$ newgrp sail
```
