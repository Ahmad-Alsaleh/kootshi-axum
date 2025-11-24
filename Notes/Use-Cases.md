// TODO: go over each model (entity) and check what operations each of the three user types (player, business, admin) need to do. write down these operations along with their endpoints.

# Milestone 1: Basic MVP (Core Booking Flow)

## Accounts & Authentication

- [x] user registers a business account. `POST /auth/signup`
- [x] user registers a player account. `POST /auth/signup`
- [x] user logs in. `POST /auth/login`
- [x] user logs out. `POST /auth/logout`
- [ ] (later) user resets password (email-based).
- [x] user updates his profile info. `PATCH /users/me`
- [x] user views his profile info. `GET /users/me`
- [x] user deletes his profile. `DELETE /users/me`

## Pitch Management (Business Accounts Only)

- [x] business creates a pitch. `POST /pitches`
- [x] business adds a time slots to a pitch. `POST /timeslots`
    - [x] insure `{pitch_id}` (from the request body) belongs to the same user
    - [x] (imp) ensure no overlaps. (use DB exclusion constraints)
- [x] business views its pitches `GET /pitches?user_id=me` // TODO: can this be designed better, maybe /my/pitches
    - [x] support filters (by location, surface type, price, availability, etc.). `GET /pitches?filters=...`

## Pitch Discovery & Booking

- [x] player views all pitches. `GET /pitches`
    - [x] guests should be able to access this endpoint
    - [x] support pagination
    - [x] by default, sort by relevance to user (user's location, favorite sport, budget, etc.) unless a flag in the query params is set to false.
    - [x] support filters (by location, surface type, price, availability, etc.). `GET /pitches?filters=...`
- [x] player views time slots of a pitch. `GET /pitches/{pitch_id}/timeslots`
- [x] player books a timeslot. `POST /bookings`

## System

> TODO: check how to acheive the first point.
> I'll use pessimistic locking for now (mainly due to simplicity).

- [x] (imp) prevent double-booking (atomic booking operation).
- [ ] verify business account ownership for pitch operations.
- [ ] send confirmation email after booking.

# Milestone 2: Better CRUD & Quality-of-Life

## Players

- [x] player cancels a booking. `DELETE /bookings/{booking_id}`
    - [x] ensures `{booking_id}` belongs to same user
- [x] player views booking history. `GET /bookings`

## Business Users

- [x] business edits pitch info. `PATCH /pitches/{patch_id}`
    - [x] ensures `{patch_id}` belongs to same user
- [x] business deletes a pitch. `DELETE /pitches/{patch_id}`
    - [x] ensures `{patch_id}` belongs to same user
- [x] business deletes availability slots. `DELETE /timeslots/{timeslot_id}`
    - [x] ensures `{timeslot_id}` belongs to same user
- [x] business updates availability slots. `PATCH timeslots/{timeslot_id}`
    - [x] ensures `{timeslot_id}` belongs to same user
- [x] business views bookings of their pitches. `GET /bookings`

## General

- [x] system soft deletes instead of hard deletes.
- [x] system prevents deletion of slots that have bookings. (use a DB constraint).
    - [x] later, we can notify the owner for confirmation. if confirmed, send an email to the players and a penaltiy/fine to the owner.
- [x] system prevents creation and deletion of slots that are in the past.
- [x] pagination for listing pitches and bookings, timeslots, etc.
- [x] global search (by area, pitch name, etc.).

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

- [x] admin creates a user `POST /users`
- [x] admin deletes a user `DELETE /users/{user_id}`
- [x] admin views basic profile info of a user. `GET /users/{user_id}`
- [x] admin updates basic profile info of a user. `PATCH /users/{user_id}`

- [x] admin creates a pitch for a company. `POST /pitches`
- [x] admin deletes a pitch of a company. `POST /pitches/{pitch_id}`
- [x] admin views pitches of a company. <----

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
