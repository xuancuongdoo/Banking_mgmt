-- Drop the foreign key constraints on the tables referencing banks and branches
ALTER TABLE accounts DROP CONSTRAINT accounts_bank_id_fkey;
ALTER TABLE accounts DROP CONSTRAINT accounts_branch_id_fkey;
ALTER TABLE beneficiaries DROP CONSTRAINT beneficiaries_bank_id_fkey;
ALTER TABLE beneficiaries DROP CONSTRAINT beneficiaries_branch_id_fkey;
ALTER TABLE cards DROP CONSTRAINT cards_bank_id_fkey;
ALTER TABLE cards DROP CONSTRAINT cards_branch_id_fkey;
ALTER TABLE customers DROP CONSTRAINT customers_bank_id_fkey;
ALTER TABLE customers DROP CONSTRAINT customers_branch_id_fkey;
ALTER TABLE loans DROP CONSTRAINT loans_bank_id_fkey;
ALTER TABLE loans DROP CONSTRAINT loans_branch_id_fkey;
ALTER TABLE refunds DROP CONSTRAINT refunds_bank_id_fkey;
ALTER TABLE refunds DROP CONSTRAINT refunds_branch_id_fkey;
ALTER TABLE transactions DROP CONSTRAINT transactions_bank_id_fkey;
ALTER TABLE transactions DROP CONSTRAINT transactions_branch_id_fkey;
ALTER TABLE transfers DROP CONSTRAINT transfers_bank_id_fkey;
ALTER TABLE transfers DROP CONSTRAINT transfers_branch_id_fkey;

-- Drop the columns added to the tables
ALTER TABLE accounts DROP COLUMN bank_id;
ALTER TABLE accounts DROP COLUMN branch_id;
ALTER TABLE beneficiaries DROP COLUMN bank_id;
ALTER TABLE beneficiaries DROP COLUMN branch_id;
ALTER TABLE cards DROP COLUMN bank_id;
ALTER TABLE cards DROP COLUMN branch_id;
ALTER TABLE customers DROP COLUMN bank_id;
ALTER TABLE customers DROP COLUMN branch_id;
ALTER TABLE loans DROP COLUMN bank_id;
ALTER TABLE loans DROP COLUMN branch_id;
ALTER TABLE refunds DROP COLUMN bank_id;
ALTER TABLE refunds DROP COLUMN branch_id;
ALTER TABLE transactions DROP COLUMN bank_id;
ALTER TABLE transactions DROP COLUMN branch_id;
ALTER TABLE transfers DROP COLUMN bank_id;
ALTER TABLE transfers DROP COLUMN branch_id;

