# Models

TODO: revisit all NOT NULL & UNIQUE modifiers for all columns
TODO: revisit all on delete/update for references
TODO: add exclusion constraints. e.g.: `ALTER TABLE Timeslot ADD EXCLUDE USING gist ( pitch_id WITH =, slot_range WITH &&)`

- User
    - id: Uuid (PK)
    - email: Email (unique, not null)
    - password_hash: String (nullable for OAuth-only accounts)
    - role: UserRole (not null)
    - phone_number: PhoneNumber
    - is_email_verified: Bool (default false)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())
    - deleted_at: TimestampTz (default null)

- PlayerProfile
    - user_id: Uuid (PK, references User.id, on delete cascade)
    - first_name: String (not null)
    - last_name: String (not null)
    - avatar_url: Url
    - skill level: Int (between 1 and 10)
    - prefered_sports: Sport\[\]
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())

- BusinessProfile
    - user_id: Uuid (PK, references User.id, on delete cascade)
    - display_name: String (unique, not null)
    - is_verified: Bool (default false)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())

- Pitch
    - id: Uuid (PK)
    - owner_id: Uuid (not null, references BusinessProfile.user_id)
    - description: String
    - sport: Sport (not null)
    - google_map_url: Url (not null)
    - is_hidden: Bool (not null, default false)
    - address_id: Uuid (not null, references Location.id)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())
    - deleted_at: TimestampTz (default null)

- Location
    - TBD

- Timeslot
    - id: Uuid (PK)
    - pitch_id: Uuid (not null, references Pitch.id, on delete cascade)
    - slot_range: TimestampTzRange (not null)
    - price: Price
    - created_by: Uuid (not null, references BusinessProfile.user_id)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())
    - deleted_at: TimestampTz (default null)

- Booking
    - id: Uuid (PK)
    - pitch_id: Uuid (not null, references Pitch.id)
    - timeslot_id: Uuid (not null, references Timeslot.id)
    - booker_id: Uuid (references PlayerProfile.user_id)
    - created_at: TimestampTz (not null, default now())
    - canceled_at: TimestampTz (default null)
    - total_fees: Price
    - remaining_fees: Price

- BookingParticipants
    - id: Uuid (PK)
    - booking_id: Uuid (references Booking.id, on delete cascade)
    - player_id: Uuid (references PlayerProfile.user_id)
    - name: String (for guests)
    - email: Email
    - phone_number: PhoneNumber
    - is_accepted: Bool (default false)

- PaymentLink
    - id: Uuid (PK)
    - booking_id: Uuid (not null, references Booking.id)
    - slug: String (unique, not null)
    - amount_cents: Int (not null, check >= 0)
    - created_at: TimestampTz (not null, default now())
    - created_by: Uuid (not null, references PlayerProfile.user_id)

# Enums

- UserRole: Player, Business, Admin
- Sport: Football, Padel

# Types

- Price 
    - AmountCents: Int
    - Currency: \[Char; 3\] (ISO)
