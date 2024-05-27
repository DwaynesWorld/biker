-- Add migration script here
CREATE TABLE IF NOT EXISTS bikes (
    id UUID NOT NULL,
    code TEXT NOT NULL,
    model TEXT NOT NULL,
    "location" geometry(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NULL,
    "status" TEXT NOT NULL,
    default_payment_method TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS rides (
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    bike_id UUID NOT NULL,
    "status" TEXT NOT NULL,
    source geometry(Point, 4326) NOT NULL,
    destination geometry(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID NOT NULL,
    ride_id UUID NOT NULL,
    amount INTEGER NOT NULL,
    "method" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(id)
);