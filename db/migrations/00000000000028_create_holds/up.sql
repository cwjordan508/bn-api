CREATE TABLE holds
(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    name TEXT NOT NULL,
    event_id uuid NOT NULL REFERENCES events(id),
    redemption_code TEXT NOT NULL,
    discount_in_cents bigint NOT NULL CHECK (discount_in_cents >= 0),
    end_at TIMESTAMP NULL,
    max_per_order BIGINT NULL CHECK (coalesce (max_per_order, 10) >= 0),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX index_holds_event_id_name ON holds (
	event_id,
	name
	);

CREATE UNIQUE INDEX index_holds_redemption_code ON holds(redemption_code);