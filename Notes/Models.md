# Models

- User
    - id: Uuid (PK)
    - email: Email (unique, not null)
    - password_hash: String (not null)
    - role: UserRole (not null)
    - phone_number: PhoneNumber
    - is_email_verified: Bool (nut null, default false)
    - avatar_url: Url
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())
    - deleted_at: TimestampTz (default null)

> Note: User.password_hash can be null in the future for OAuth-only accounts

- PlayerProfile
    - user_id: Uuid (PK, references User.id, on delete cascade, check User.role = 'Player' where User.id = PlayerProfile.user_id)
    - first_name: String (not null)
    - last_name: String (not null)
    - skill_level: Int (check between 1 and 10)
    - prefered_sports: Sport\[\]
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())

- BusinessProfile
    - user_id: Uuid (PK, references User.id, on delete cascade, check User.role = 'Business' where User.id = BusinessProfile.user_id)
    - display_name: String (unique, not null)
    - is_verified: Bool (default false)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())

- Pitch
    - id: Uuid (PK)
    - owner_id: Uuid (not null, references BusinessProfile.user_id, on delete cascade)
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
    - timeslot: TimestampTzRange (not null)
    - fees: Price
    - is_booked: Bool (not null, default false)
    - created_by: Uuid (not null, references BusinessProfile.user_id)
    - created_at: TimestampTz (not null, default now())
    - updated_at: TimestampTz (not null, default now())
    - deleted_at: TimestampTz (default null)

> EXCLUDE USING gist (pitch_id WITH =, timeslot WITH &&)

- Booking
    - id: Uuid (PK)
    - timeslot_id: Uuid (not null, unique, references Timeslot.id)
    - booked_by: Uuid (not null, references PlayerProfile.user_id)
    - created_at: TimestampTz (not null, default now())
    - canceled_at: TimestampTz (default null)
    - remaining_fees: Price

- BookingParticipant
    - id: Uuid (PK)
    - booking_id: Uuid (references Booking.id, on delete cascade)
    - player_id: Uuid (references PlayerProfile.user_id)
    - guest_id: Uuid (references GuestProfile.id)
    - accepted_invitation: Bool (default false)

> CHECK (player_id IS NOT NULL AND guest_id IS NULL) OR (player_id IS NULL AND guest_id IS NOT NULL)

- GuestProfile
    - id: Uuid (PK)
    - name: String (not null)
    - email: Email (not null)
    - phone_number: PhoneNumber (not null)

- PaymentLink
    - id: Uuid (PK)
    - booking_id: Uuid (not null, references Booking.id)
    - slug: String (unique, not null)
    - amount_cents: Int (not null, check >= 0)
    - created_by: Uuid (not null, references PlayerProfile.user_id)
    - created_at: TimestampTz (not null, default now())
    - canceled_at: TimestampTz (default null)

# Enums

- UserRole: Player, Business, Admin
- Sport: Football, Padel

# Types

- Price 
    - AmountCents: Int
    - Currency: \[Char; 3\] (ISO)
