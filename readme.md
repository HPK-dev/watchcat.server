# watchcat.server

The backend of our project `watchcat` for 2024 Taiwan science fair.

## API

| method | route | args | desc |
|---|---|---|---|
| POST | `/user_reg` | empty | registe a user .|
| GET | `/user_auth` | `card_id` => card id | Check if this card can be approved |
| POST | `/token_login` | private/unknown ( ask Google ) | Google login callback |

## Database

### table `card`
| key | desc |
|---|---|
| id | primary key, store each card |
| owner | the card owner |

### table `user`

| key | desc |
|---|---|
| user_id | primary key, the unique user id |
