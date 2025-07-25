CREATE TABLE IF NOT EXISTS hyper_candle_snapshot (
    candle_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    symbol VARCHAR(10) NOT NULL,
    interval VARCHAR(5) NOT NULL,
    open_time BIGINT NOT NULL,
    close_time BIGINT NOT NULL,
    open_price NUMERIC(20,8) NOT NULL,
    close_price NUMERIC(20,8) NOT NULL,
    low_price NUMERIC(20,8) NOT NULL,
    volume NUMERIC(30,10) NOT NULL,
    number_of_trades BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
