CREATE TABLE orderbook_delta (
    id SERIAL PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    price_dollars VARCHAR(6) NOT NULL,
    delta_fp VARCHAR(15) NOT NULL,
    side VARCHAR(3) NOT NULL,
    ts TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE orderbook_snapshot (
    id SERIAL PRIMARY KEY NOT NULL,
    ts TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    market_ticker VARCHAR(26) NOT NULL,
    yes_dollars_fp JSON,
    no_dollars_fp JSON
);

CREATE TABLE trades (
    id Serial PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    yes_price_dollars VARCHAR(6) NOT NULL,
    no_price_dollars VARCHAR(6) NOT NULL,
    count_fp VARCHAR(15) NOT NULL,
    taker_side VARCHAR(2) NOT NULL,
    taker_outcome_side VARCHAR(2) NOT NULL,
    taker_book_side VARCHAR(3) NOT NULL,
    ts_ms BIGINT
);

CREATE TABLE market_ticker (
    id Serial PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    price_dollars VARCHAR(6) NOT NULL,
    yes_price_dollars VARCHAR(6) NOT NULL,
    no_price_dollars VARCHAR(6) NOT NULL,
    volume_fp VARCHAR(20) NOT NULL,
    ts_ms TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);



