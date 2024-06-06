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
  - `room_id` (String) _(Optional)_
  - `user_id` (String) _(Optional)_
  - `begins` (String) _(Optional)_
  - `ends` (String) _(Optional)_
  - `approval_pending` (Boolean) _(Optional)_
  - `description` (String) _(Optional)_
- **Description:**  
   Retrieves reservations from the database based on the provided filters.  
   Returns `200 OK` with a JSON array of reservation objects(type reference are below) matching the specified criteria.
  ```typescript
  interface Reservation {
    reservation_id: number;
    room_id: string;
    username: string;
    user_id: string;
    description?: string;
    begins: string;
    ends: string;
    approval_pending: boolean;
  }
  ```

---

### `/reserve` **PATCH**

- **Arguments:**
  - `reservation_id` (Integer)
  - `room_id` (String) _(Optional)_
  - `user_id` (String) _(Optional)_
  - `begins` (String) _(Optional)_
  - `ends` (String) _(Optional)_
  - `approval_pending` (Boolean) _(Optional)_
  - `description` (String) _(Optional)_
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

---

### `/room_status` **GET**

- **Arguments:**
  - `room_id` (String) _(Optional)_
  - `user_id` (String) _(Optional)_
  - `begins` (String) _(Optional)_
  - `ends` (String) _(Optional)_
- **Description:**  
  Retrieves card records from the database based on the provided filters.  
  Returns `200 OK` a JSON array of reservation objects matching the specified criteria.

## Database Schema

### Users Table

The `Users` table stores information about users.

- **id** `text` `NOT NULL`: Unique identifier for each user.
- **email** `text`: Email address of the user.
- **name** `text`: Name of the user.

---

### Cards Table

The `Cards` table stores information about cards.

- **expire** `DATETIME` `NULL`: Expiration date and time of the card.
- **owner** `text`: Identifier of the card owner.
- **id** `char(8)` `PRIMARY KEY` `NOT NULLL`: Unique identifier for each card.

---

### Reservations Table

The `Reservations` table stores information about room reservations.

- **reservation_id**`INT` `AUTO_INCREMENT` `PRIMARY KEY`: Unique identifier for each reservation.
- **room_id** `text` `NOT NULL`: Identifier of the reserved room.
- **user_id** `text` `NOT NULL`: Identifier of the user making the reservation.
- **description** `LONGTEXT` `NOT NULL`: Description or notes about the reservation.
- **begins** `DATETIME` `NOT NULL`: Start date and time of the reservation.
- **ends** `DATETIME` `NOT NULL`: End date and time of the reservation.
- **approval_pending** `BOOLEAN` `DEFAULT` `TRUE`: Indicates whether the reservation is pending approval.

---

### Rooms Table

The `Rooms` table stores information about rooms.

- **room_id** `text` `NOT NULL` `PRIMARY KEY`: Identifier of the room.
- **device_mac** `char(17)` `NOT NULL` `UNIQUE`: MAC address of the device associated with the room.

---

### Records Table

The `Records` table stores information about card swipes.

- **record_id** `INT` `AUTO_INCREMENT` `PRIMARY KEY`: Unique identifier for each record.
- **room_id** `text` `NOT NULL`: Identifier of the room where the card was swiped.
- **device_mac** `char(17)` `NOT NULL`: MAC address of the device where the card was swiped.
- **card_id** `char(8)` `NOT NULL`: Identifier of the card that was swiped.
- **at** `DATETIME` `NOT NULL`: Date and time when the card was swiped.
