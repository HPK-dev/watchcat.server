
# watchcat.server

The backend of our project `watchcat` for 2024 Taiwan science fair.

## How to run


### Using script (only avalable on Linux)

```bash
cd /watchcat-server
chmod +x ./install.sh
./install.sh
```

### Run manually (with docker)

1. set following variables  in `.env.docker`

    - "BIND_IP"
    - "BIND_PORT" 
    - "GOOGLE_OAUTH_CLIENT_SECERT" 
    - "GOOGLE_OAUTH_CLIENT_ID" 
    - "PG_DATABASE_URL" 

2. build in docker
    ```bash
    sudo docker compose up
    ```
    or some way make docker compose run depend on your OS


### Run manually (without docker)

Ensure you have Rust installed.

1. set following variables  in `.env`

    - "BIND_IP"
    - "BIND_PORT" 
    - "GOOGLE_OAUTH_CLIENT_SECERT" 
    - "GOOGLE_OAUTH_CLIENT_ID" 
    - "PG_DATABASE_URL" 

2. run
    ```bash
    cargo run --release
    ```
