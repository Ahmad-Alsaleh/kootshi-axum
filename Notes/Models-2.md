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

- player creates an account.
- player views personal profile info.
- player updates personal profile info.
- player deletes personal profile info.

- business creates an account.
- business views personal profile info.
- business updates personal profile info.
- business deletes personal profile info.

- admin views personal profile info.
- admin updates personal profile info.
- admin deletes personal profile info.

- admin creates a user profile.
- admin views profile info of a user.
- admin updates profile info of a user.
- admin deletes profile info of a user.

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

- player views relevant pitches (based on location, prefered sports, etc.).

- business creates a personal pitch.
- business views personal pitches.
- business updates a personal pitch.
- business deletes personal pitches.

- admin creates a pitch for a business.
- admin views pitches of a business.
- admin updates a pitch for a business.
- admin deletes a pitch of a business.

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

- player views timeslots of a pitch.
- player views all relevant timeslots (ig can be same endpoint as above, but use filters in query params).

- business creates a timeslot for a personal pitch.
- business views all personal timeslots.
- business views personal timeslots of a pitch (ig can be same endpoint as above, but use filters in query params).
- business updates a personal timeslot.
- business deletes a personal timeslot.

- admin creates a timeslot.
- admin views timeslots of a pitch.
- admin views timeslots of a business (ig can be same endpoint as above, but use filters in query params).
- admin updates a timeslot.
- admin deletes a timeslot.

## Booking

### Attributes

- id: Uuid (PK).
- timeslot_id: Uuid (not null, unique, references Timeslot.id).
- booked_by: Uuid (not null, references PlayerProfile.user_id).
- created_at: TimestampTz (not null, default now()).
- canceled_at: TimestampTz (default null).
- remaining_fees: Price.

### Operations

- player creates a booking.
- player views personal bookings (i.e. booking history).
- player deletes a booking (i.e. cancells the booking).

- business views bookings of their pitches.

- admin creates a booking.
- admin views bookings of all companies.
- admin views bookings of a company.
- admin deletes a booking.

## BookingParticipant

### Attributes

- id: Uuid (PK).
- booking_id: Uuid (references Booking.id, on delete cascade).
- player_id: Uuid (references PlayerProfile.user_id).
- guest_id: Uuid (references GuestProfile.id).
- accepted_invitation: Bool (default false).

> CHECK (player_id IS NOT NULL AND guest_id IS NULL) OR (player_id IS NULL AND guest_id IS NOT NULL)
> in the future, add a flag to the booking to allow random participants to join. in this case, anyone can view all participants of that booking

- player views participants of a booking he made or is invited to (guests can see a live view using a unique link, later).
- player updates info of participants for a booking he made.
- player deletes a participant of a booking he made.
- player delets himself as a participant (i.e., withdraws from the booking/match or decline the invitaion).

- business views the number of participants for a booking of one of their pitches.

- admin creates participants.
- admin views participants.
- admin updates info of participants for a specific booking.
- admin deletes participants from a specific booking.

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
