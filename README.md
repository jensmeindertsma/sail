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

## TEMPORARY

```rust
// main.rs
mod application;

use application::Application;

fn main() -> impl Termination {  
    let runtime = match Builder::new_multi_thread()
    .enable_all()
    .build() {
        Ok(runtime) => runtime,
        Err(io_error) => {
            tracing::error!("failed to set up multi thread runtime: {io_error}");
            return ExitCode::FAILURE;
        }
    }
    
    let runtime.block_on(async move {
        let application = Application::start()?;
    }).instrument(info_span!("application"))

    ExitCode::SUCESS
}

async fn run()

// application.rs
use std::error::Error;

struct Application {
    configuration: Arc<Configuration>
    state: State
}

impl Application {

    pub async fn start() -> Result<Self, StartupError> {
        let configuration = Configuration::load().map_err(StartupError::Configuration)?;
        let 
    }
}

pub enum StartupError {
    Configuration
    State
}

impl Display for StartupError {
    pub fn fmt(/* ... */) -> /* ... */ {
        /* ... */
    }
}

impl Error for StartupError {}
```