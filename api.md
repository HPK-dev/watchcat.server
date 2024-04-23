# API Documentation

## Endpoints

### `/card_login` **GET**

- **Arguments:**
  - `card_id` (String)
  - `device_mac` (String)
- **Description:**  
  Checks if the provided card is eligible for approval. Returns `200 OK` if the card is approved.

---

### `/token_login` **POST**

- **Arguments:**
  - `access_token` (String)
- **Description:**  
  Handles Google login callbacks, setting login cookies, and updating the database for new users.  
  Returns `302 Found` upon successful login.

---

### `/reserve` **PUT**

- **Arguments:**
  - `room_id` (String)
  - `user_id` (String)
  - `description` (String)
  - `begins` (String)
  - `ends` (String)
- **Description:**  
  Creates a new reservation in the database for a specified room and user within a specified time range.  
  Returns `201 Created` if the reservation is successful.

---

### `/reserve` **GET**

- **Arguments:**
  - `room_id` (String) *(Optional)*
  - `user_id` (String) *(Optional)*
  - `begins` (String) *(Optional)*
  - `ends` (String) *(Optional)*
  - `approval_pending` (Boolean) *(Optional)*
  - `description` (String) *(Optional)*
- **Description:**  
  Retrieves reservations from the database based on the provided filters.  
  Returns `200 OK` a JSON array of reservation objects matching the specified criteria.
---

### `/reserve` **PATCH**

- **Arguments:**
  - `reservation_id` (Integer)
  - `room_id` (String) *(Optional)*
  - `user_id` (String) *(Optional)*
  - `begins` (String) *(Optional)*
  - `ends` (String) *(Optional)*
  - `approval_pending` (Boolean) *(Optional)*
  - `description` (String) *(Optional)*
- **Description:**  
  Updates an existing reservation in the database with the provided fields.  
  If all fields are `null`, returns `400 Bad Request`.  
  Checks for conflicts with other reservations and returns `409 Conflict` if detected.  
Returns `200 OK` if successful.

---

### `/reserve` **DELETE**

- **Arguments:**
  - `reservation_id` (String)
- **Description:**  
  Deletes the reservation with the specified ID from the database.  
  Returns `204 No Content` if the deletion is successful.


## Database Schema


### Users Table

The `Users` table stores information about users.

- **id**: Unique identifier for each user. It is of type `text` and cannot be null.
- **email**: Email address of the user. It is of type `text` and can be null.
- **name**: Name of the user. It is of type `text` and can be null.

---

### Cards Table

The `Cards` table stores information about cards.

- **expire**: Expiration date and time of the card. It is of type `DATETIME` and can be null.
- **owner**: Identifier of the card owner. It is of type `text`.
- **id**: Unique identifier for each card. It is of type `char(8)` and cannot be null. It is the primary key of the table.

---

### Reservations Table

The `Reservations` table stores information about room reservations.

- **reservation_id**: Unique identifier for each reservation. It is an auto-incremented integer and serves as the primary key.
- **room_id**: Identifier of the reserved room. It is of type `text` and cannot be null.
- **user_id**: Identifier of the user making the reservation. It is of type `text` and cannot be null.
- **description**: Description or notes about the reservation. It is of type `LONGTEXT` and can be null.
- **begins**: Start date and time of the reservation. It is of type `DATETIME` and cannot be null.
- **ends**: End date and time of the reservation. It is of type `DATETIME` and cannot be null.
- **approval_pending**: Indicates whether the reservation is pending approval. It is of type `BOOLEAN` and defaults to `TRUE`.

---

### Rooms Table

The `Rooms` table stores information about rooms.

- **room_id**: Identifier of the room. It is of type `text` and cannot be null.
- **device_mac**: MAC address of the device associated with the room. It is of type `char(12)` and serves as the primary key of the table. It cannot be null.

