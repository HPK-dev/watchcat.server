<!--
TODO: Need a rewrite
-->

## API

| method | route | args | desc |
|---|---|---|---|
| GET | `/card_login` | `card_id`: ID of card <br/> `device_mac`: MAC of card reader | Check if this card can be approved |
| POST | `/token_login` | `access_token`: Access token for Google OAuth authentication | Google login callback |

## Database

### table `Cards`
| key | type | desc |
|-----|------|------|
| id | char(8) PRIMARY KEY | the unique card id |
| owner | text | the card owner |
| expire | DATETIME (No timezone offset)| card expired time |

### table `Users`

| key | type | desc |
|-----|------|------|
| id | text |  the user id |
| email | text | this user's email |

