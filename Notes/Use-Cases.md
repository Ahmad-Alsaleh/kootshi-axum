# Milestone 1: Basic MVP (Core Booking Flow)

## Accounts & Authentication

- user registers a business account
- user registers a player account
- user logs in / logs out
- user resets password (email-based)
- user updates basic profile info (email, name, phone)

## Pitch Management (Business Accounts Only)

- user creates a pitch
- user adds availability time slots
    - ensure no overlaps

## Pitch Discovery & Booking

- player views all pitches
- player filters pitches (location, surface type, price, availability)
- player views time slots
- player books a time slot

## System

> TODO: check how to acheive the first point
- prevent double-booking (atomic booking operation)
- verify business account ownership for pitch operations
- email confirmation on booking

# Milestone 2: Better CRUD & Quality-of-Life

## Players

- player cancels a booking
- player views booking history

## Business Users

- business edits pitch info
- business deletes a pitch
- business deletes or updates availability slots

## General

- system soft deletes instead of hard deletes (audit trail)
- system prevents deletion of slots that have bookings
    - later, we can notify the owner for confirmation. if confirmed, send an email to the players and a penaltiy/fine to the owner 
- system prevents deletion of slots that are in the past
- business views bookings for their pitches
- pagination for listing pitches and bookings
- global search (by area, pitch name, etc.)

# Milestone 3: Payments (Multi-Party Split + Tracking)

## Payments

- player pays full amount
- player pays partial amount
- payment link generation (shareable public link)
- external players pay through link (no login required)
- player pays two-installment (in-advance deposit + remaining)
    - confirm booking after paying first installment
    - close the invoice after paying the seconds installment

## Payment Infrastructure

- payment status tracking (pending / confirmed / failed / refunded)
- prevent bookings without completed or valid payment rules
- automatic expiry of unpaid bookings
- webhooks integration (Stripe, PayPal, etc.)
- business dashboard for payout history
- player view: payment receipts, invoices
- ability to issue refunds (admin/business accounts)
- aggregate payment totals per business and pay once periodically

## Security / Fraud

- enforce max number of split payments
- validate player emails/numbers for splits
- prevent overpayments or double-charging

# Milestone 4: Game Rank Matching (Social + Skill Discovery)

## Player Profiles & Ranking

- players set their skill level / rank
- system updates rank based on play history
- players can rate each other after games
- automated rank adjustments
- display rank on player profile

## Matching / Team Formation

- players can create a match request (day, time, location, skill range)
- system matches players with similar ranks
- system suggests open pitches based on player availability
- team auto-balancer (2 teams with similar total rank)

## Post-Match System

- collect attendance confirmation
- final rank recalculation after match
- match history for each player

# Milestone 5: Notifications & Communication

## Notifications

- email/SMS/push notifications for:
  - booking confirmations
  - booking changes / cancellations
  - payment confirmations
  - payment reminders
  - match reminders

# Milestone 6: Admin & Platform Management

## Admin Portal

- manage users (ban, verify, view logs)
- manage business accounts (approve/verify)
- manage disputes & refunds
- audit logs for edits, bookings, and payments
- view platform analytics (bookings per day, revenue, etc.)

# Milestone 7: Scalability & Optimization

- rate limiting
- caching search results & pitch lists
- background jobs (sending emails, clearing expired bookings, processing webhooks)
- monitoring + logging
- load testing
