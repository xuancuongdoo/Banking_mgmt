-- Add down migration script here
DROP TABLE IF EXISTS beneficiaries;

-- Drop the cards table
DROP TABLE IF EXISTS cards;

-- Drop the accounts table
DROP TABLE IF EXISTS accounts;

-- Drop the customers table
DROP TABLE IF EXISTS customers;

DROP TABLE IF EXISTS loans;

DROP TABLE IF EXISTS refunds;

DROP TABLE IF EXISTS transactions;

DROP TABLE IF EXISTS transfers;

-- Drop the ENUM types
DROP TYPE IF EXISTS accounttype;

DROP TYPE IF EXISTS cardstatus;

DROP TYPE IF EXISTS cardtype;

DROP TYPE IF EXISTS status;
