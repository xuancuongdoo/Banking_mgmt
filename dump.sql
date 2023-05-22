

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


CREATE TYPE public.accounttype AS ENUM (
    'checking',
    'savings',
    'credit'
);


ALTER TYPE public.accounttype OWNER TO postgres;


CREATE TYPE public.cardstatus AS ENUM (
    'active',
    'inactive',
    'closed'
);


ALTER TYPE public.cardstatus OWNER TO postgres;


CREATE TYPE public.cardtype AS ENUM (
    'debit',
    'credit'
);


ALTER TYPE public.cardtype OWNER TO postgres;


CREATE TYPE public.status AS ENUM (
    'pending',
    'approved',
    'rejected',
    'close'
);


ALTER TYPE public.status OWNER TO postgres;


CREATE TYPE public.transaction_type AS ENUM (
    'Repay Loan',
    'Repay Interest',
    'Peer-to-Peer',
    'Cash Withdrawal',
    'Cash Deposit',
    'Debit Card Charge'
);


ALTER TYPE public.transaction_type OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;


CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;


CREATE TABLE public.accounts (
    id uuid NOT NULL,
    account_number character varying(255) NOT NULL,
    balance integer NOT NULL,
    opened_date date NOT NULL,
    last_updated_date date NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    account_type public.accounttype NOT NULL,
    customer_id uuid NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.accounts OWNER TO postgres;


CREATE TABLE public.banks (
    id uuid NOT NULL,
    bank_name character varying(255) NOT NULL,
    fee integer NOT NULL,
    total_money integer DEFAULT 0 NOT NULL,
    total_debt_to_collect integer DEFAULT 0 NOT NULL,
    total_loans_given integer DEFAULT 0 NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.banks OWNER TO postgres;


CREATE TABLE public.beneficiaries (
    id uuid NOT NULL,
    customer_id uuid NOT NULL,
    beneficiary_name character varying(255) NOT NULL,
    beneficiary_account_number character varying(255) NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.beneficiaries OWNER TO postgres;


CREATE TABLE public.branches (
    id uuid NOT NULL,
    branch_name character varying(255) NOT NULL,
    bank_id uuid NOT NULL,
    pre_deposit_amount integer NOT NULL,
    total_money integer DEFAULT 0 NOT NULL,
    debt_to_collect integer DEFAULT 0 NOT NULL,
    loans_given integer DEFAULT 0 NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.branches OWNER TO postgres;


CREATE TABLE public.cards (
    id uuid NOT NULL,
    card_number character varying(255) NOT NULL,
    account_number character varying(255) NOT NULL,
    expiration_date date NOT NULL,
    cvv character varying(255) NOT NULL,
    issued_date date NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    balance integer NOT NULL,
    card_status public.cardstatus NOT NULL,
    card_type public.cardtype NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.cards OWNER TO postgres;


CREATE TABLE public.customers (
    id uuid NOT NULL,
    customer_name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    phone_number character varying(255) NOT NULL,
    cic_number character varying(255) NOT NULL,
    inserted_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.customers OWNER TO postgres;


CREATE TABLE public.loans (
    id uuid NOT NULL,
    amount integer NOT NULL,
    interest_rate numeric NOT NULL,
    start_date date NOT NULL,
    end_date date NOT NULL,
    status public.status NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    lender_card_number character varying(255) NOT NULL,
    borrower_card_number character varying(255) NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.loans OWNER TO postgres;


CREATE TABLE public.refunds (
    id uuid NOT NULL,
    transaction_id uuid NOT NULL,
    refund_amount integer NOT NULL,
    refund_date date NOT NULL,
    status public.status NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.refunds OWNER TO postgres;


CREATE TABLE public.transactions (
    id uuid NOT NULL,
    account_number character varying(255) NOT NULL,
    card_number character varying(255) NOT NULL,
    amount integer NOT NULL,
    transaction_date date NOT NULL,
    status public.status NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    transaction_type public.transaction_type NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.transactions OWNER TO postgres;


CREATE TABLE public.transfers (
    id uuid NOT NULL,
    sender_card_number character varying(255) NOT NULL,
    beneficiary_account_number character varying(255) NOT NULL,
    amount integer NOT NULL,
    transfer_date date NOT NULL,
    inserted_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    bank_id uuid NOT NULL,
    branch_id uuid NOT NULL
);


ALTER TABLE public.transfers OWNER TO postgres;


ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_account_number_key UNIQUE (account_number);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_customer_id_key UNIQUE (customer_id);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.banks
    ADD CONSTRAINT banks_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.beneficiaries
    ADD CONSTRAINT beneficiaries_beneficiary_account_number_key UNIQUE (beneficiary_account_number);



ALTER TABLE ONLY public.beneficiaries
    ADD CONSTRAINT beneficiaries_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.branches
    ADD CONSTRAINT branches_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_card_number_key UNIQUE (card_number);



ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customers_cic_number_key UNIQUE (cic_number);



ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customers_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.loans
    ADD CONSTRAINT loans_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.refunds
    ADD CONSTRAINT refunds_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.transfers
    ADD CONSTRAINT transfers_pkey PRIMARY KEY (id);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES public.customers(id);



ALTER TABLE ONLY public.beneficiaries
    ADD CONSTRAINT beneficiaries_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.beneficiaries
    ADD CONSTRAINT beneficiaries_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.beneficiaries
    ADD CONSTRAINT beneficiaries_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES public.customers(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.branches
    ADD CONSTRAINT branches_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_account_number_fkey FOREIGN KEY (account_number) REFERENCES public.accounts(account_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customers_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customers_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.loans
    ADD CONSTRAINT loans_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.loans
    ADD CONSTRAINT loans_borrower_card_number_fkey FOREIGN KEY (borrower_card_number) REFERENCES public.cards(card_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.loans
    ADD CONSTRAINT loans_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.loans
    ADD CONSTRAINT loans_lender_card_number_fkey FOREIGN KEY (lender_card_number) REFERENCES public.cards(card_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.refunds
    ADD CONSTRAINT refunds_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.refunds
    ADD CONSTRAINT refunds_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.refunds
    ADD CONSTRAINT refunds_transaction_id_fkey FOREIGN KEY (transaction_id) REFERENCES public.transactions(id) ON DELETE CASCADE;



ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_account_number_fkey FOREIGN KEY (account_number) REFERENCES public.accounts(account_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_card_number_fkey FOREIGN KEY (card_number) REFERENCES public.cards(card_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.transfers
    ADD CONSTRAINT transfers_bank_id_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id);



ALTER TABLE ONLY public.transfers
    ADD CONSTRAINT transfers_beneficiary_account_number_fkey FOREIGN KEY (beneficiary_account_number) REFERENCES public.beneficiaries(beneficiary_account_number) ON DELETE CASCADE;



ALTER TABLE ONLY public.transfers
    ADD CONSTRAINT transfers_branch_id_fkey FOREIGN KEY (branch_id) REFERENCES public.branches(id);



ALTER TABLE ONLY public.transfers
    ADD CONSTRAINT transfers_sender_card_number_fkey FOREIGN KEY (sender_card_number) REFERENCES public.cards(card_number) ON DELETE CASCADE;



