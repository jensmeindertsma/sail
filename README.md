# sail

Seamless self-owned application deployment.

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
