-- Add down migration script here
-- Revert the changes made to the customers table
ALTER TABLE customers

ALTER COLUMN customer_name DROP NOT NULL,
ALTER COLUMN email DROP NOT NULL,
ALTER COLUMN phone_number DROP NOT NULL,
ALTER COLUMN cic_number DROP NOT NULL,
ALTER COLUMN inserted_at DROP NOT NULL,
ALTER COLUMN updated_at DROP NOT NULL;

-- Revert the changes made to the accounts table
ALTER TABLE accounts

ALTER COLUMN account_number DROP NOT NULL,
ALTER COLUMN balance DROP NOT NULL,
ALTER COLUMN opened_date DROP NOT NULL,
ALTER COLUMN last_updated_date DROP NOT NULL,
ALTER COLUMN account_type DROP NOT NULL,
ALTER COLUMN customer_id DROP NOT NULL;

-- Revert the changes made to the cards table
ALTER TABLE cards

ALTER COLUMN card_number DROP NOT NULL,
ALTER COLUMN account_number DROP NOT NULL,
ALTER COLUMN expiration_date DROP NOT NULL,
ALTER COLUMN cvv DROP NOT NULL,
ALTER COLUMN issued_date DROP NOT NULL,
ALTER COLUMN balance DROP NOT NULL,
ALTER COLUMN card_status DROP NOT NULL,
ALTER COLUMN card_type DROP NOT NULL;

-- Revert the changes made to the beneficiaries table
ALTER TABLE beneficiaries

ALTER COLUMN customer_id DROP NOT NULL,
ALTER COLUMN beneficiary_name DROP NOT NULL,
ALTER COLUMN beneficiary_account_number DROP NOT NULL;

-- Revert the changes made to the transactions table
ALTER TABLE transactions

ALTER COLUMN account_number DROP NOT NULL,
ALTER COLUMN card_number DROP NOT NULL,
ALTER COLUMN transaction_type DROP NOT NULL,
ALTER COLUMN amount DROP NOT NULL,
ALTER COLUMN transaction_date DROP NOT NULL,
ALTER COLUMN status DROP NOT NULL;

-- Revert the changes made to the refunds table
ALTER TABLE refunds

ALTER COLUMN transaction_id DROP NOT NULL,
ALTER COLUMN refund_amount DROP NOT NULL,
ALTER COLUMN refund_date DROP NOT NULL,
ALTER COLUMN status DROP NOT NULL;

-- Revert the changes made to the loans table
ALTER TABLE loans

ALTER COLUMN amount DROP NOT NULL,
ALTER COLUMN interest_rate DROP NOT NULL,
ALTER COLUMN start_date DROP NOT NULL,
ALTER COLUMN end_date DROP NOT NULL,
ALTER COLUMN status DROP NOT NULL,
ALTER COLUMN lender_card_number DROP NOT NULL,
ALTER COLUMN borrower_card_number DROP NOT NULL;

-- Revert the changes made to the transfers table
ALTER TABLE transfers

ALTER COLUMN sender_card_number DROP NOT NULL,
ALTER COLUMN beneficiary_account_number DROP NOT NULL,
ALTER COLUMN amount DROP NOT NULL,
ALTER COLUMN transfer_date DROP NOT NULL;
