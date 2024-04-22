## API Documentation

### `/card_login` **GET**

- **Arguments:**
  - `card_id` (String)
  - `device_mac` (String)
- **Description:**  
  Checks if the provided card is eligible for approval. Returns `200 OK` if the card is approved.

### `/token_login` **POST**

- **Arguments:**
  - `access_token` (String)
- **Description:**  
  Handles Google login callbacks, setting login cookies, and updating the database for new users.  
  Returns `302 Found` upon successful login.

### `/reserve` **PUT**

- **Arguments:**
  - `room_id` (String)
  - `user_id` (String)
  - `description` (String)
  - `begins` (String)
  - `ends` (String)
- **Description:**  
  Creates a new reservation in the database for a specified room and user within a specified time range.  
  Returns `200 OK` if the reservation is successful.

### `/reserve` **GET**

- **Optional Arguments:**
  - `room_id` (String)
  - `user_id` (String)
  - `begin` (String)
  - `ends` (String)
  - `approval_pending` (Boolean)
  - `description` (String)
- **Description:**  
  Retrieves reservations from the database based on the provided filters.  
  Returns `200 OK` a JSON array of reservation objects matching the specified criteria.

### `/reserve` **POST**

- **Arguments:**
  - `reservation_id` (Integer)
  - `room_id` (String) *(Optional)*
  - `user_id` (String) *(Optional)*
  - `begin` (String) *(Optional)*
  - `ends` (String) *(Optional)*
  - `approval_pending` (Boolean) *(Optional)*
  - `description` (String) *(Optional)*
- **Description:**  
  Updates an existing reservation in the database with the provided fields.   
  Checks for conflicts with other reservations and returns `409 Conflict` if detected. Returns `200 OK` if successful.


### `/reserve` **DELETE**

- **Arguments:**
  - `reservation_id` (String)
- **Description:**  
  Deletes the reservation with the specified ID from the database.  
  Returns `200 OK` if the deletion is successful.
