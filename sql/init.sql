--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2 (Debian 15.2-1.pgdg110+1)
-- Dumped by pg_dump version 16.1 (Ubuntu 16.1-1.pgdg22.04+1)

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

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: pg_database_owner
--

CREATE SCHEMA IF NOT EXISTS public;


ALTER SCHEMA public OWNER TO pg_database_owner;

--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: pg_database_owner
--

COMMENT ON SCHEMA public IS 'standard public schema';


--
-- Name: token; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.token AS (
	amount bigint,
	"tokenId" text
);


ALTER TYPE public.token OWNER TO postgres;

--
-- Name: get_token_info(bigint, bigint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.get_token_info(t_id bigint, t_amount bigint) RETURNS TABLE(token_info json)
    LANGUAGE plpgsql STABLE COST 200000 PARALLEL SAFE
    AS $$
BEGIN
RETURN QUERY
WITH liquidity_pool as (
	select * from spectrum.spectrum_boxes where box_id = (select max(box_id) from spectrum.spectrum_boxes where quote_id = t_id) order by erg_value desc limit 1
)
select row_to_json(t_info.*)
from (
select
	t_amount as token_amount,
	ARRAY(SELECT public.get_wrapped_tokens(
		t.id,
		t_amount
	)) as wrapped_tokens,
	t.*,
	coalesce(
		case when t_id = 0 then 1 else null end,
		(lp.base_amount/10^9)/(lp.quote_amount/10^t.decimals),
		0
	) as value_in_erg
from
public.tokens t
left join liquidity_pool lp on lp.quote_id = t.id
where t.id = t_id
) t_info;

END;
$$;


ALTER FUNCTION public.get_token_info(t_id bigint, t_amount bigint) OWNER TO postgres;

--
-- Name: get_token_supply(bigint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.get_token_supply(t_id bigint) RETURNS TABLE(non_circulating bigint, liquid bigint, locked bigint)
    LANGUAGE plpgsql STABLE COST 200000 PARALLEL SAFE
    AS $$
BEGIN
RETURN QUERY
with non_circ as (
	select id from public.addresses a where address in (
		'9hXmgvzndtakdSAgJ92fQ8ZjuKirWAw8tyDuyJrXP6sKHVpCz8XbMANK3BVJ1k3WD6ovQKTCasjKL5WMncRB6V9HvmMnJ2WbxYYjtLFS9sifDNXJWugrNEgoVK887bR5oaLZA95yGkMeXVfanxpNDZYaXH9KpHCpC5ohDtaW1PF17b27559toGVCeCUNti7LXyXV8fWS1mVRuz2PhLq5mB7hg2bqn7CZtVM8ntbUJpjkHUc9cP1R8Gvbo1GqcNWgM7gZkr2Dp514BrFz1cXMkv7TYEqH3cdxX9c82hH6fdaf3n6avdtZ5bgqerUZVDDW6ZsqxrqTyTMQUUirRAi3odmMGmuMqDJbU3Z1VnCF9NBow7jrKUDSgckDZakFZNChsr5Kq1kQyNitYJUh9fra1jLHCQ9yekz3te9E'
	,'HNLdwoHRsUSevguzRajzvy1DLAvUJ9YgQezQq6GGZiY4TmU9VDs2ae8mRpQkfEnLmuUKyJibZD2bXR2yoo1p8T5WCRKPn4rJVJ2VR2LvRBk8ViCmhcume5ubWaySXTUqpftEaaURTM6KSFxe4QbRFbToyPzZ3JJmjoDn4WzHh5ioXZMj7AX6xTwJvFmzPuko9BqDk5z1RJtD1wP4kd8sSsLN9P2YNQxmUGDEBYHaDCoAhY7Pg5oKit6ZyqMynoiycWqctfg1EHhMUKCTJsZNnidU961ri98RaYP4CfEwYQ3d9dRVuC6S1n7J1wPPHYqmUBgJCGWbTULayXUowSSmRuZUkQYGo9vvNaEpB7ManiLsX1n8cBYwN4XoVsY24mCfptBP86P4rZ5fgcr9mYtQ9nG934DMDZBbjs81VzCupB6KVrGCe1WtYSr6c1DwkNAinBMwqcqxTznXZUvfBsjDSCtJzCut44xcc7Zsy9mWz2B2pqhdKsX83BVzMDDM5hnjXTShYfauJGs81'
	,'MUbV38YgqHy7XbsoXWF5z7EZm524Ybdwe5p9WDrbhruZRtehkRPT92imXer2eTkjwPDfboa1pR3zb3deVKVq3H7Xt98qcTqLuSBSbHb7izzo5jphEpcnqyKJ2xhmpNPVvmtbdJNdvdopPrHHDBbAGGeW7XYTQwEeoRfosXzcDtiGgw97b2aqjTsNFmZk7khBEQywjYfmoDc9nUCJMZ3vbSspnYo3LarLe55mh2Np8MNJqUN9APA6XkhZCrTTDRZb1B4krgFY1sVMswg2ceqguZRvC9pqt3tUUxmSnB24N6dowfVJKhLXwHPbrkHViBv1AKAJTmEaQW2DN1fRmD9ypXxZk8GXmYtxTtrj3BiunQ4qzUCu1eGzxSREjpkFSi2ATLSSDqUwxtRz639sHM6Lav4axoJNPCHbY8pvuBKUxgnGRex8LEGM8DeEJwaJCaoy8dBw9Lz49nq5mSsXLeoC4xpTUmp47Bh7GAZtwkaNreCu74m9rcZ8Di4w1cmdsiK1NWuDh9pJ2Bv7u3EfcurHFVqCkT3P86JUbKnXeNxCypfrWsFuYNKYqmjsix82g9vWcGMmAcu5nagxD4iET86iE2tMMfZZ5vqZNvntQswJyQqv2Wc6MTh4jQx1q2qJZCQe4QdEK63meTGbZNNKMctHQbp3gRkZYNrBtxQyVtNLR8xEY8zGp85GeQKbb37vqLXxRpGiigAdMe3XZA4hhYPmAAU5hpSMYaRAjtvvMT3bNiHRACGrfjvSsEG9G2zY5in2YWz5X9zXQLGTYRsQ4uNFkYoQRCBdjNxGv6R58Xq74zCgt19TxYZ87gPWxkXpWwTaHogG1eps8WXt8QzwJ9rVx6Vu9a5GjtcGsQxHovWmYixgBU8X9fPNJ9UQhYyAWbjtRSuVBtDAmoV1gCBEPwnYVP5GCGhCocbwoYhZkZjFZy6ws4uxVLid3FxuvhWvQrVEDYp7WRvGXbNdCbcSXnbeTrPMey1WPaXX'
	)
)
select
sum(case when addr_type = 0 then token_amount else 0 end)::bigint as non_circulating_supply,
sum(case when addr_type = 1 then token_amount else 0 end)::bigint as liquid_supply,
sum(case when addr_type = 2 then token_amount else 0 end)::bigint as locked_supply
from (
select aib.*,
(case when aib.address = any(select id from non_circ) then 0
when length(a.address) = 51 and starts_with(a.address, '9') then 1
else 2
end) as addr_type
from public.asset_in_box aib
join public.addresses a on a.id = aib.address
where aib.token_id = t_id
and aib.spent is null
) as main;

END;
$$;


ALTER FUNCTION public.get_token_supply(t_id bigint) OWNER TO postgres;

--
-- Name: get_wrapped_tokens(bigint, bigint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.get_wrapped_tokens(t_id bigint, t_amount bigint) RETURNS TABLE(wrapped_tokens json)
    LANGUAGE plpgsql STABLE PARALLEL SAFE
    AS $$
BEGIN
RETURN QUERY
select T.token_info from (
select get_token_info(wrapped_token_id, round(t_amount*wrapped_amount)::bigint) as token_info
from public.wrapped
where token_id = t_id
and box_id = (select max(box_id) from public.wrapped where token_id = t_id)
and timepoint = (select max(timepoint) from public.wrapped where token_id = t_id)
) T;
END;
$$;


ALTER FUNCTION public.get_wrapped_tokens(t_id bigint, t_amount bigint) OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: addresses; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.addresses (
    id bigint NOT NULL,
    address text NOT NULL,
    ergotree text NOT NULL,
    ergotree_hash text NOT NULL,
    ergotree_template text NULL
);


ALTER TABLE public.addresses OWNER TO postgres;

--
-- Name: asset_in_box; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.asset_in_box (
    id bigint NOT NULL,
    box_id bigint NOT NULL,
    transaction_id bigint NOT NULL,
    token_id bigint NOT NULL,
    token_amount bigint NOT NULL,
    address bigint NOT NULL,
    created integer NOT NULL,
    spent integer,
    index integer
);


ALTER TABLE public.asset_in_box OWNER TO postgres;

--
-- Name: asset_in_:box_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."asset_in_:box_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."asset_in_:box_id_seq" OWNER TO postgres;

--
-- Name: asset_in_:box_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."asset_in_:box_id_seq" OWNED BY public.asset_in_box.id;


--
-- Name: blocks; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.blocks (
    height integer NOT NULL,
    header_id text NOT NULL,
    "timestamp" timestamp without time zone NOT NULL,
    header jsonb NOT NULL
);


ALTER TABLE public.blocks OWNER TO postgres;

--
-- Name: boxes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.boxes (
    id bigint NOT NULL,
    address bigint NOT NULL,
    data jsonb NOT NULL,
    created integer NOT NULL,
    spent integer,
    box_id text NOT NULL,
    transaction_id bigint
);


ALTER TABLE public.boxes OWNER TO postgres;

--
-- Name: data_inputs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.data_inputs (
    id bigint NOT NULL,
    box_id bigint NOT NULL,
    transaction_id bigint NOT NULL,
    index integer NOT NULL
);


ALTER TABLE public.data_inputs OWNER TO postgres;

--
-- Name: inputs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.inputs (
    id bigint NOT NULL,
    box_id bigint NOT NULL,
    transaction_id bigint NOT NULL,
    spending_proof jsonb NOT NULL,
    index integer NOT NULL
);


ALTER TABLE public.inputs OWNER TO postgres;

--
-- Name: token_in_box; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.token_in_box (
    id bigint NOT NULL,
    box_id bigint NOT NULL,
    token_id bigint NOT NULL,
    amount bigint NOT NULL,
    index integer NOT NULL
);


ALTER TABLE public.token_in_box OWNER TO postgres;

--
-- Name: tokens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tokens (
    id bigint NOT NULL,
    token_id text NOT NULL,
    token_name text NOT NULL,
    token_description text NOT NULL,
    issuer_box bigint NOT NULL,
    issuance_box bigint NOT NULL,
    decimals integer,
    minted bigint
);


ALTER TABLE public.tokens OWNER TO postgres;

--
-- Name: transactions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.transactions (
    id bigint NOT NULL,
    transaction_id text NOT NULL,
    height integer NOT NULL
);


ALTER TABLE public.transactions OWNER TO postgres;

--
-- Name: wrapped; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.wrapped (
    id bigint NOT NULL,
    box_id bigint NOT NULL,
    token_id bigint NOT NULL,
    wrapped_token_id bigint NOT NULL,
    wrapped_amount double precision NOT NULL,
    type text NOT NULL,
    sub_type text,
    timepoint timestamp without time zone NOT NULL
);


ALTER TABLE public.wrapped OWNER TO postgres;

--
-- Name: wrapped_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.wrapped_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.wrapped_id_seq OWNER TO postgres;

--
-- Name: wrapped_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.wrapped_id_seq OWNED BY public.wrapped.id;


--
-- Name: asset_in_box id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asset_in_box ALTER COLUMN id SET DEFAULT nextval('public."asset_in_:box_id_seq"'::regclass);


--
-- Name: wrapped id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wrapped ALTER COLUMN id SET DEFAULT nextval('public.wrapped_id_seq'::regclass);


--
-- Name: addresses addresses_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.addresses
    ADD CONSTRAINT addresses_pkey PRIMARY KEY (id);


--
-- Name: asset_in_box asset_in_:box_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asset_in_box
    ADD CONSTRAINT "asset_in_:box_pkey" PRIMARY KEY (id);


--
-- Name: blocks blocks_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.blocks
    ADD CONSTRAINT blocks_pkey PRIMARY KEY (height);


--
-- Name: boxes boxes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT boxes_pkey PRIMARY KEY (id);


--
-- Name: data_inputs data_inputs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.data_inputs
    ADD CONSTRAINT data_inputs_pkey PRIMARY KEY (id);


--
-- Name: inputs inputs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inputs
    ADD CONSTRAINT inputs_pkey PRIMARY KEY (id);


--
-- Name: token_in_box token_in_box_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_in_box
    ADD CONSTRAINT token_in_box_pkey PRIMARY KEY (id);


--
-- Name: tokens tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens_pkey PRIMARY KEY (id);


--
-- Name: transactions transactions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);


--
-- Name: wrapped wrapped_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wrapped
    ADD CONSTRAINT wrapped_pkey PRIMARY KEY (id);


--
-- Name: asset_in_box_address_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX asset_in_box_address_idx ON public.asset_in_box USING btree (address, spent);


--
-- Name: asset_in_box_spent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX asset_in_box_spent_idx ON public.asset_in_box USING btree (spent);


--
-- Name: asset_in_box_token_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX asset_in_box_token_id_idx ON public.asset_in_box USING btree (token_id, spent, created);


--
-- Name: asset_in_box_transaction_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX asset_in_box_transaction_id_idx ON public.asset_in_box USING btree (transaction_id);


--
-- Name: boxes_spent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX boxes_spent_idx ON public.boxes USING btree (spent, id);


--
-- Name: fki_fk_boxes_created; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_fk_boxes_created ON public.boxes USING btree (created);


--
-- Name: fki_fk_boxes_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_fk_boxes_spent ON public.boxes USING btree (spent);


--
-- Name: fki_inputs__transactions_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_inputs__transactions_fk ON public.inputs USING btree (transaction_id);


--
-- Name: fki_token_in_box__boxes_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_token_in_box__boxes_fk ON public.token_in_box USING btree (box_id);


--
-- Name: fki_tokens__boxes_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_tokens__boxes_fk ON public.tokens USING btree (issuance_box);


--
-- Name: fki_transactions__blocks_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_transactions__blocks_fk ON public.transactions USING btree (height);


--
-- Name: idx_addresses_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_address ON public.addresses USING hash (address);


--
-- Name: idx_addresses_ergotree; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_ergotree ON public.addresses USING hash (ergotree);


CREATE INDEX addresses_ergotree_template_idx ON public.addresses USING hash (ergotree_template);

--
-- Name: idx_asset_in_box__box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_asset_in_box__box_id ON public.asset_in_box USING btree (box_id);


--
-- Name: idx_blocks_timestamp; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_blocks_timestamp ON public.blocks USING btree ("timestamp");


--
-- Name: idx_boxes__transaction; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_boxes__transaction ON public.boxes USING btree (transaction_id);


--
-- Name: idx_boxes_address_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_boxes_address_box_id ON public.boxes USING btree (address, id);


--
-- Name: idx_boxes_address_created_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_boxes_address_created_spent ON public.boxes USING btree (address, created, spent);


--
-- Name: idx_boxes_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_boxes_box_id ON public.boxes USING btree (box_id);


--
-- Name: idx_boxes_box_id_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_boxes_box_id_spent ON public.boxes USING btree (box_id, spent);


--
-- Name: idx_id_created_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_id_created_spent ON public.boxes USING btree (id, created, spent);


--
-- Name: idx_token_id_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_token_id_box_id ON public.wrapped USING btree (token_id, box_id DESC);


--
-- Name: idx_token_in_box_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_token_in_box_box_id ON public.token_in_box USING btree (box_id);


--
-- Name: idx_token_in_box_token_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_token_in_box_token_id ON public.token_in_box USING btree (token_id) INCLUDE (box_id);


--
-- Name: idx_wrapped_type_sub_type; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_wrapped_type_sub_type ON public.wrapped USING btree (type, sub_type, timepoint);


--
-- Name: idz_tokens_token_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idz_tokens_token_id ON public.tokens USING btree (token_id);


--
-- Name: inputs_box_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX inputs_box_id_idx ON public.inputs USING btree (box_id);


--
-- Name: wrapped_token_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX wrapped_token_id_idx ON public.wrapped USING btree (token_id, timepoint);


--
-- Name: asset_in_box asset_in_box_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asset_in_box
    ADD CONSTRAINT asset_in_box_fk FOREIGN KEY (box_id) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: asset_in_box asset_in_box_spent_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asset_in_box
    ADD CONSTRAINT asset_in_box_spent_fk FOREIGN KEY (spent) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: boxes fk_boxes_created; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT fk_boxes_created FOREIGN KEY (created) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- Name: boxes fk_boxes_spent; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT fk_boxes_spent FOREIGN KEY (spent) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE SET NULL NOT VALID;


--
-- Name: inputs inputs__transactions_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inputs
    ADD CONSTRAINT inputs__transactions_fk FOREIGN KEY (transaction_id) REFERENCES public.transactions(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- Name: token_in_box token_in_box__boxes_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_in_box
    ADD CONSTRAINT token_in_box__boxes_fk FOREIGN KEY (box_id) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- Name: tokens tokens__boxes_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens__boxes_fk FOREIGN KEY (issuance_box) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- Name: transactions transactions__blocks_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions__blocks_fk FOREIGN KEY (height) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- Name: wrapped wrapped_box_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wrapped
    ADD CONSTRAINT wrapped_box_id_fkey FOREIGN KEY (box_id) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: wrapped wrapped_token_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wrapped
    ADD CONSTRAINT wrapped_token_id_fkey FOREIGN KEY (token_id) REFERENCES public.tokens(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: wrapped wrapped_wrapped_token_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.wrapped
    ADD CONSTRAINT wrapped_wrapped_token_id_fkey FOREIGN KEY (wrapped_token_id) REFERENCES public.tokens(id);


--
-- PostgreSQL database dump complete
--


--
-- Mempool Tables
--

-- Mempool transactions
CREATE TABLE public.mempool_transactions (
    id BIGSERIAL PRIMARY KEY,
    transaction_id TEXT NOT NULL UNIQUE,
    data JSONB NOT NULL,
    first_seen TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mempool_tx_id ON public.mempool_transactions USING hash (transaction_id);

-- Mempool boxes (outputs)
CREATE TABLE public.mempool_boxes (
    id BIGSERIAL PRIMARY KEY,
    box_id TEXT NOT NULL UNIQUE,
    address BIGINT NOT NULL,
    data JSONB NOT NULL,
    mempool_transaction_id BIGINT NOT NULL REFERENCES public.mempool_transactions(id) ON DELETE CASCADE,
    output_index INTEGER NOT NULL
);

CREATE INDEX idx_mempool_boxes_box_id ON public.mempool_boxes USING hash (box_id);
CREATE INDEX idx_mempool_boxes_address ON public.mempool_boxes USING btree (address);
CREATE INDEX idx_mempool_boxes_tx_id ON public.mempool_boxes USING btree (mempool_transaction_id);

-- Mempool inputs
CREATE TABLE public.mempool_inputs (
    id BIGSERIAL PRIMARY KEY,
    mempool_transaction_id BIGINT NOT NULL REFERENCES public.mempool_transactions(id) ON DELETE CASCADE,
    box_id TEXT NOT NULL,
    confirmed_box_id BIGINT,
    spending_proof JSONB NOT NULL,
    index INTEGER NOT NULL
);

CREATE INDEX idx_mempool_inputs_box_id ON public.mempool_inputs USING hash (box_id);
CREATE INDEX idx_mempool_inputs_tx_id ON public.mempool_inputs USING btree (mempool_transaction_id);
CREATE INDEX idx_mempool_inputs_confirmed ON public.mempool_inputs USING btree (confirmed_box_id) WHERE confirmed_box_id IS NOT NULL;

-- Mempool tokens (for tokens minted in mempool)
CREATE TABLE public.mempool_tokens (
    id BIGSERIAL PRIMARY KEY,
    token_id TEXT NOT NULL UNIQUE,
    token_name TEXT,
    token_description TEXT,
    decimals INTEGER,
    minted BIGINT,
    issuer_box TEXT NOT NULL,
    issuance_box BIGINT NOT NULL REFERENCES public.mempool_boxes(id) ON DELETE CASCADE
);

CREATE INDEX idx_mempool_tokens_token_id ON public.mempool_tokens USING hash (token_id);

-- Mempool tokens in boxes
CREATE TABLE public.mempool_token_in_box (
    id BIGSERIAL PRIMARY KEY,
    mempool_box_id BIGINT NOT NULL REFERENCES public.mempool_boxes(id) ON DELETE CASCADE,
    token_id BIGINT,
    mempool_token_id BIGINT,
    token_id_str TEXT NOT NULL,
    amount BIGINT NOT NULL,
    index INTEGER NOT NULL,
    CONSTRAINT chk_mempool_token_ref CHECK (token_id IS NOT NULL OR mempool_token_id IS NOT NULL)
);

CREATE INDEX idx_mempool_tib_box ON public.mempool_token_in_box USING btree (mempool_box_id);
CREATE INDEX idx_mempool_tib_token_str ON public.mempool_token_in_box USING hash (token_id_str);
