-- Drop the existing transaction_type column


-- Alter accounts table
ALTER TABLE accounts
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE accounts ADD CONSTRAINT accounts_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT accounts_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter beneficiaries table
ALTER TABLE beneficiaries
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE beneficiaries ADD CONSTRAINT beneficiaries_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT beneficiaries_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter cards table
ALTER TABLE cards
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE cards ADD CONSTRAINT cards_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT cards_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter customers table
ALTER TABLE customers
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE customers ADD CONSTRAINT customers_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT customers_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter loans table
ALTER TABLE loans
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE loans ADD CONSTRAINT loans_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT loans_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter refunds table
ALTER TABLE refunds
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL;

ALTER TABLE refunds ADD CONSTRAINT refunds_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT refunds_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter transactions table
ALTER TABLE transactions
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL,
ADD CONSTRAINT transactions_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT transactions_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);

-- Alter transfers table
ALTER TABLE transfers
ADD COLUMN bank_id uuid NOT NULL,
ADD COLUMN branch_id uuid NOT NULL,
ADD CONSTRAINT transfers_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES banks (id),
ADD CONSTRAINT transfers_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES branches (id);
