# Models

// TODO: (later) give admin priveleges, i.e. what tables/models can he read from and write to (right now admins have full power, they can read and write to everything)
// TODO: (later) add operations done by guests (right now only players, businesses, and admins are considered)

## User

### Attributes

- id: Uuid (PK).
- email: Email (unique, not null).
- password_hash: String (not null).
- role: UserRole (not null).
- phone_number: PhoneNumber.
- is_email_verified: Bool (nut null, default false).
- avatar_url: Url.
- created_at: TimestampTz (not null, default now()).
- updated_at: TimestampTz (not null, default now()).
- deleted_at: TimestampTz (default null).

> Note: User.password_hash can be null in the future for OAuth-only accounts

### Operations

- player creates an account. `POST /auth/signup`
- player views personal profile info. `GET /users/me`
- player updates personal profile info. `PATCH /users/me`
- player deletes personal profile info. `DELETE /users/me`

- business creates an account. `POST /auth/signup`
- business views personal profile info. `GET /users/me`
- business updates personal profile info. `PATCH /users/me`
- business deletes personal profile info. `DELETE /users/me`

- admin views personal profile info. `GET /users/me`
- admin updates personal profile info. `PATCH /users/me`
- admin deletes personal profile info. `DELETE /users/me`

- admin creates a user profile. `POST /users`
- admin views profile info of a user. `GET /users/{user_id}`
- admin updates profile info of a user. `PATCH /users/{user_id}`
- admin deletes profile info of a user. `DELETE /users/{user_id}`

## PlayerProfile

### Attributes

- user_id: Uuid (PK, references User.id, on delete cascade, check User.role = 'Player' where User.id = PlayerProfile.user_id).
- first_name: String (not null).
- last_name: String (not null).
- skill_level: Int (check between 1 and 10).
- prefered_sports: Sport\[\].

### Operations

- all operations of User should apply here as well.

## BusinessProfile

### Attributes

- user_id: Uuid (PK, references User.id, on delete cascade, check User.role = 'Business' where User.id = BusinessProfile.user_id).
- display_name: String (unique, not null).
- is_verified: Bool (default false).

### Operations

- all operations of User should apply here as well.

- business verifies their account.

## Pitch

### Attributes

- id: Uuid (PK).
- owner_id: Uuid (not null, references BusinessProfile.user_id, on delete cascade).
- description: String.
- sport: Sport (not null).
- google_map_url: Url (not null).
- is_hidden: Bool (not null, default false).
- address_id: Uuid (not null, references Location.id).
- created_at: TimestampTz (not null, default now()).
- updated_at: TimestampTz (not null, default now()).
- deleted_at: TimestampTz (default null).

### Operations

- player views relevant pitches (based on location, prefered sports, etc.). `GET /pitches`

- business creates a personal pitch.  `POST /pitches`
- business views personal pitches. `GET /pitches`
- business updates a personal pitch. `PATCH /pitches/{pitch_id}`
- business deletes a personal pitch. `DELETE /pitches/{pitch_id}`

- admin creates a pitch for a business. `POST /pitches`
- admin views pitches of a business. `GET /pitches`
- admin updates a pitch for a business. `PATCH /pitches/{pitch_id}`
- admin deletes a pitch for a business. `DELETE /pitches/{pitch_id}`

## Location

### Attributes

- id: Uuid (PK).
- address_line_1: String (not null).
- address_line_2: String.
- city: String (not null).
- postal_code: String.
- country: String (not null).
- coordinates: GEOMETRY/GEOGRAPHY (not null, for spatial indexing).

## Timeslot

### Attributes

- id: Uuid (PK).
- pitch_id: Uuid (not null, references Pitch.id, on delete cascade).
- timeslot: TimestampTzRange (not null).
- fees: Price.
- is_booked: Bool (not null, default false).
- created_at: TimestampTz (not null, default now()).
- updated_at: TimestampTz (not null, default now()).
- deleted_at: TimestampTz (default null).

> EXCLUDE USING gist (pitch_id WITH =, timeslot WITH &&)

### Operations

- player views timeslots of a pitch. `GET /timeslots?pitch_id={pitch_id}`
- player views all relevant timeslots. `GET /timeslots`

- business creates a timeslot for a personal pitch. `POST /timeslots`
- business views all personal timeslots. `GET /timeslots`
- business views personal timeslots of a pitch. `GET /timeslots?pitch_id={pitch_id}`
- business updates a personal timeslot. `PATCH /timeslots/{timeslot_id}`
- business deletes a personal timeslot. `DELETE /timeslots/{timeslot_id}`

- admin creates a timeslot. `POST /timeslots`
- admin views timeslots of a pitch. `GET /timeslots?pitch_id={pitch_id}`
- admin views timeslots of a business. `GET /timeslots?business={business_id}`
- admin updates a timeslot. `PATCH /timeslots/{timeslot_id}`
- admin deletes a timeslot. `DELETE /timeslots/{timeslot_id}`

## Booking

### Attributes

- id: Uuid (PK).
- timeslot_id: Uuid (not null, unique, references Timeslot.id).
- booked_by: Uuid (not null, references PlayerProfile.user_id).
- created_at: TimestampTz (not null, default now()).
- canceled_at: TimestampTz (default null).
- remaining_fees: Price.

### Operations

- player creates a booking. `POST /bookings`
- player views personal bookings (i.e. booking history). `GET /bookings`
- player deletes a booking (i.e. cancells the booking). `DELETE /bookings/{booking_id}`

- business views bookings of their pitches. `GET /bookings`

- admin creates a booking. `POST /bookings`
- admin views bookings of all companies. `GET /bookings`
- admin views bookings of a company. `GET /bookings?business_id={business_id}`
- admin deletes a booking. `DELETE /bookings/{booking_id}`

## BookingParticipant

### Attributes

- id: Uuid (PK).
- booking_id: Uuid (references Booking.id, on delete cascade).
- player_id: Uuid (references PlayerProfile.user_id).
- guest_id: Uuid (references GuestProfile.id).
- accepted_invitation: Bool (default false).

> CHECK (player_id IS NOT NULL AND guest_id IS NULL) OR (player_id IS NULL AND guest_id IS NOT NULL)
> in the future, add a flag to the booking to allow random participants to join. in this case, anyone can view all participants of that booking

### Operations

- player views participants of a booking he made or is invited to (guests can see a live view using a unique link, later). `GET /bookings/{booking_id}/participants`
- player updates info of participants for a booking he made. `PATCH /bookings/{booking_id}/participants`
- player deletes a participant of a booking he made. `DELETE /bookings/{booking_id}/participants/{participant_id}`
- player delets himself as a participant (i.e., withdraws from the booking/match or decline the invitaion). `DELETE /bookings/{booking_id}/participants/me`

- business views the number of participants for a booking of one of their pitches. `GET /bookings/{booking_id}/participants`

- admin creates participants. `POST /bookings/{booking_id}/participants`
- admin views participants. `GET /bookings/{booking_id}/participants`
- admin updates info of participants for a specific booking. `PATCH /bookings/{booking_id}/participants`
- admin deletes participants from a specific booking. `DELETE /bookings/{booking_id}/participants`

## GuestProfile

### Attributes

- id: Uuid (PK).
- name: String (not null).
- email: Email (not null).
- phone_number: PhoneNumber (not null).

## PaymentLink

### Attributes

- id: Uuid (PK).
- booking_id: Uuid (not null, references Booking.id).
- slug: String (unique, not null).
- amount_cents: Int (not null, check >= 0).
- created_at: TimestampTz (not null, default now()).
- canceled_at: TimestampTz (default null).

# Enums

- UserRole: Player, Business, Admin.
- Sport: Football, Padel.

# Types

- Price.
    - AmountCents: Int
    - Currency: \[Char; 3\] (ISO)
