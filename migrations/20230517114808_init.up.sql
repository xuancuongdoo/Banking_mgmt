-- Add up migration script here
-- Create ENUM types if they don't exist
CREATE TYPE accounttype AS ENUM ('checking', 'savings', 'credit');

CREATE TYPE cardstatus AS ENUM ('active', 'inactive', 'closed');

CREATE TYPE cardtype AS ENUM ('debit', 'credit');

CREATE TYPE status AS ENUM ('pending', 'approved', 'rejected', 'close');

CREATE TABLE IF NOT EXISTS customers (
    id UUID PRIMARY KEY,
    customer_name VARCHAR(255),
    email VARCHAR(255),
    phone_number VARCHAR(255),
    cic_number VARCHAR(255) UNIQUE -- Add unique cic_number column
);

-- Create the accounts table if it doesn't exist
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    account_number VARCHAR(255) UNIQUE,
    balance INTEGER,
    opened_date DATE,
    last_updated_date DATE,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE,
    account_type accounttype,
    customer_id UUID UNIQUE REFERENCES customers(id) -- Add unique customer_id foreign key
);


-- Create the cards table if it doesn't exist
CREATE TABLE IF NOT EXISTS cards (
    id UUID PRIMARY KEY,
    card_number VARCHAR(255) UNIQUE,
    account_number VARCHAR(255) REFERENCES accounts(account_number) ON DELETE CASCADE,
    expiration_date DATE,
    cvv VARCHAR(255),
    issued_date DATE,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE,
    balance INTEGER,
    card_status cardstatus,
    card_type cardtype
);

-- Create the beneficiaries table if it doesn't exist
CREATE TABLE IF NOT EXISTS beneficiaries (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    -- Add customer_id column
    beneficiary_name VARCHAR(255),
    beneficiary_account_number VARCHAR(255) UNIQUE,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE
);
-- Create the transactions table if it doesn't exist
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    account_number VARCHAR(255) REFERENCES accounts(account_number) ON DELETE CASCADE,
    card_number VARCHAR(255) REFERENCES cards(card_number) ON DELETE CASCADE,
    transaction_type VARCHAR(255),
    amount INTEGER,
    transaction_date DATE,
    status status,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE
);
-- Create the refunds table if it doesn't exist
CREATE TABLE IF NOT EXISTS refunds (
    id UUID PRIMARY KEY,
    transaction_id UUID REFERENCES transactions(id) ON DELETE CASCADE,
    refund_amount INTEGER,
    refund_date DATE,
    status status,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE
);

-- Create the loans table if it doesn't exist
CREATE TABLE IF NOT EXISTS loans (
    id UUID PRIMARY KEY,
    amount INTEGER,
    interest_rate NUMERIC,
    start_date DATE,
    end_date DATE,
    status status,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE,
    lender_card_number VARCHAR(255) REFERENCES cards(card_number) ON DELETE CASCADE,
    borrower_card_number VARCHAR(255) REFERENCES cards(card_number) ON DELETE CASCADE
);



-- Create the transfers table if it doesn't exist
CREATE TABLE IF NOT EXISTS transfers (
    id UUID PRIMARY KEY,
    sender_card_number VARCHAR(255) REFERENCES cards(card_number) ON DELETE CASCADE,
    beneficiary_account_number VARCHAR(255) REFERENCES beneficiaries(beneficiary_account_number) ON DELETE CASCADE,
    amount INTEGER,
    transfer_date DATE,
    inserted_at TIMESTAMP WITHOUT TIME ZONE,
    updated_at TIMESTAMP WITHOUT TIME ZONE
);

