
# watchcat.server

The backend of our project `watchcat` for 2024 Taiwan science fair.

## Build

### Linux:
### Automatic build using script
    cd /watchcat-server
    chmod +x ./install.sh
    ./install.sh
### Build Manually
#### 1.set "BIND_IP","BIND_PORT","GOOGLE_OAUTH_KEY","GOOGLE_OAUTH_ID","DATABASE_URL" in ```.env.docker```
#### 2.build in docker
    sudo docker compose up
