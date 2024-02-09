

## API

| method | route | args | desc |
|---|---|---|---|
| GET | `/card_login` | `card_id` => card id | Check if this card can be approved |
| POST | `/token_login` | private/unknown ( ask Google ) | Google login callback |

## Database

### table `Cards`
| key | desc |
|---|---|
| id | primary key, store each card |
| owner | the card owner |

### table `Users`

| key | desc |
|---|---|
|  id | primary key, the unique user id |
| email | this user's email |
| expire | card expired time |
