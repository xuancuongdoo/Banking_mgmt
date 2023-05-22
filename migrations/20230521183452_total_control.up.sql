-- Add up migration script here
-- Add the total_cards, total_accounts, total_transactions, and total_customers columns to the branches table
ALTER TABLE branches
ADD COLUMN total_cards integer DEFAULT 0 NOT NULL,
ADD COLUMN total_accounts integer DEFAULT 0 NOT NULL,
ADD COLUMN total_transactions integer DEFAULT 0 NOT NULL,
ADD COLUMN total_customers integer DEFAULT 0 NOT NULL;

-- Add the total_cards, total_accounts, total_transactions, and total_customers columns to the banks table
ALTER TABLE banks
ADD COLUMN total_cards integer DEFAULT 0 NOT NULL,
ADD COLUMN total_accounts integer DEFAULT 0 NOT NULL,
ADD COLUMN total_transactions integer DEFAULT 0 NOT NULL,
ADD COLUMN total_customers integer DEFAULT 0 NOT NULL;
