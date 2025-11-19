// TODO: go over each model (entity) and check what operations each of the three user types (player, business, admin) need to do. write down these operations along with their endpoints.

# Milestone 1: Basic MVP (Core Booking Flow)

## Accounts & Authentication

- [ ] user registers a business account. `POST /auth/signup`
- [ ] user registers a player account. `POST /auth/signup`
- [ ] user logs in. `POST /auth/login`
- [ ] user logs out. `POST /auth/logout`
- [ ] (later) user resets password (email-based).
- [ ] user updates his profile info. `PATCH /users/me`
- [ ] user views his profile info. `GET /users/me`
- [ ] user deletes his profile. `DELETE /users/me`

## Pitch Management (Business Accounts Only)

- [ ] business creates a pitch. `POST /pitches`
- [ ] business adds a time slots to a pitch. `POST /timeslots`
    - [ ] insure `{pitch_id}` (from the request body) belongs to the same user
    - [ ] (imp) ensure no overlaps. (use DB exclusion constraints)
- [ ] business views its pitches `GET /pitches?user_id=me` // TODO: can this be designed better, maybe /my/pitches
    - [ ] support filters (by location, surface type, price, availability, etc.). `GET /pitches?filters=...`

## Pitch Discovery & Booking

- [ ] player views all pitches. `GET /pitches`
    - [ ] guests should be able to access this endpoint
    - [ ] support pagination
    - [ ] by default, sort by relevance to user (user's location, favorite sport, budget, etc.) unless a flag in the query params is set to false.
    - [ ] support filters (by location, surface type, price, availability, etc.). `GET /pitches?filters=...`
- [ ] player views time slots of a pitch. `GET /pitches/{pitch_id}/timeslots`
- [ ] player books a timeslot. `POST /bookings`

## System

> TODO: check how to acheive the first point.
> I'll use pessimistic locking for now (mainly due to simplicity).

- [ ] (imp) prevent double-booking (atomic booking operation).
- [ ] verify business account ownership for pitch operations.
- [ ] send confirmation email after booking.

# Milestone 2: Better CRUD & Quality-of-Life

## Players

- [ ] player cancels a booking. `DELETE /bookings/{booking_id}`
    - [ ] ensures `{booking_id}` belongs to same user
- [ ] player views booking history. `GET /bookings`

## Business Users

- [ ] business edits pitch info. `PATCH /pitches/{patch_id}`
    - [ ] ensures `{patch_id}` belongs to same user
- [ ] business deletes a pitch. `DELETE /pitches/{patch_id}`
    - [ ] ensures `{patch_id}` belongs to same user
- [ ] business deletes availability slots. `DELETE /timeslots/{timeslot_id}`
    - [ ] ensures `{timeslot_id}` belongs to same user
- [ ] business updates availability slots. `PATCH timeslots/{timeslot_id}`
    - [ ] ensures `{timeslot_id}` belongs to same user
- [ ] business views bookings of their pitches. `GET /bookings`

## General

- [ ] system soft deletes instead of hard deletes.
- [ ] system prevents deletion of slots that have bookings. (use a DB constraint).
    - [ ] later, we can notify the owner for confirmation. if confirmed, send an email to the players and a penaltiy/fine to the owner.
- [ ] system prevents creation and deletion of slots that are in the past.
- [ ] pagination for listing pitches and bookings, timeslots, etc.
- [ ] global search (by area, pitch name, etc.).

# Milestone 3: Payments (Multi-Party Split + Tracking)

## Payments

- [ ] player pays full amount.
- [ ] player pays partial amount.
- [ ] payment link generation (shareable public link).
- [ ] external players pay through link (no login required).
- [ ] player pays two-installment (in-advance deposit + remaining).
    - [ ] confirm booking after paying first installment.
    - [ ] close the invoice after paying the seconds installment.

## Payment Infrastructure

- [ ] payment status tracking (pending/confirmed/failed/refunded).
- [ ] prevent bookings without completed or valid payment rules.
- [ ] automatic expiry of unpaid bookings.
- [ ] webhooks integration (Stripe, PayPal, etc).
- [ ] business dashboard for payout history.
- [ ] player view: payment receipts, invoices.
- [ ] ability to issue refunds (admin/business accounts).
- [ ] aggregate payment totals per business and pay once periodically.

## Security / Fraud

- [ ] enforce max number of split payments.
- [ ] validate player emails/numbers for splits.
- [ ] prevent overpayments or double-charging.

# Milestone 4: Game Rank Matching (Social + Skill Discovery)

## Player Profiles & Ranking

- [ ] players set their skill level/rank.
- [ ] system updates rank based on play history.
- [ ] players can rate each other after games.
- [ ] automated rank adjustments.
- [ ] display rank on player profile.

## Matching / Team Formation

- [ ] players can create a match request (day, time, location, skill range).
- [ ] system matches players with similar ranks.
- [ ] system suggests open pitches based on player availability.
- [ ] team auto-balancer (2 teams with similar total rank).

## Post-Match System

- [ ] collect attendance confirmation.
- [ ] final rank recalculation after match.
- [ ] match history for each player.

# Milestone 5: Notifications & Communication

## Notifications

- [ ] email/SMS/push notifications for:
  - [ ] booking confirmations.
  - [ ] booking changes / cancellations.
  - [ ] payment confirmations.
  - [ ] payment reminders.
  - [ ] match reminders.

# Milestone 6: Admin & Platform Management

- [ ] admin creates a user `POST /users`
- [ ] admin deletes a user `DELETE /users/{user_id}`
- [ ] admin views basic profile info of a user. `GET /users/{user_id}`
- [ ] admin updates basic profile info of a user. `PATCH /users/{user_id}`

- [ ] admin creates a pitch for a company. `POST /pitches`
- [ ] admin deletes a pitch of a company. `POST /pitches/{pitch_id}`
- [ ] admin views pitches of a company. <----

- [ ] manage users (ban, verify, view logs).
- [ ] manage business accounts (approve/verify).
- [ ] manage disputes & refunds.
- [ ] audit logs for edits, bookings, and payments.
- [ ] view platform analytics (bookings per day, revenue, etc.).

# Milestone 7: Scalability & Optimization

- [ ] rate limiting.
- [ ] caching search results & pitch lists.
- [ ] background jobs (sending emails, clearing expired bookings, processing webhooks).
- [ ] monitoring + logging.
- [ ] load testing.
