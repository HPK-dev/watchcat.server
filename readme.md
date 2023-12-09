# watchcat.server

The backend of our project `watchcat` for 2024 Taiwan science fair.

## API

| method | route | args | desc |
|---|---|---|---|
| POST | `/user_reg` | empty | registe a user .|
| GET | `/user_auth` | `cid` => card id | Check if this card can be approved |
