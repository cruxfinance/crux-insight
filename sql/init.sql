--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1 (Debian 15.1-1.pgdg110+1)
-- Dumped by pg_dump version 15.4 (Ubuntu 15.4-1.pgdg22.04+1)

-- Started on 2023-08-14 14:20:55 CEST

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


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 217 (class 1259 OID 58504)
-- Name: addresses; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.addresses (
    id bigint NOT NULL,
    address text NOT NULL,
    ergotree text NOT NULL,
    ergotree_hash text NOT NULL
);


ALTER TABLE public.addresses OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 58485)
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
-- TOC entry 216 (class 1259 OID 58495)
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
-- TOC entry 220 (class 1259 OID 58531)
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
-- TOC entry 219 (class 1259 OID 58522)
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
-- TOC entry 222 (class 1259 OID 58547)
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
-- TOC entry 221 (class 1259 OID 58538)
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
-- TOC entry 218 (class 1259 OID 58513)
-- Name: transactions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.transactions (
    id bigint NOT NULL,
    transaction_id text NOT NULL,
    height integer NOT NULL
);


ALTER TABLE public.transactions OWNER TO postgres;

--
-- TOC entry 3222 (class 2606 OID 58511)
-- Name: addresses addresses_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.addresses
    ADD CONSTRAINT addresses_pkey PRIMARY KEY (id);


--
-- TOC entry 3212 (class 2606 OID 58491)
-- Name: blocks blocks_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.blocks
    ADD CONSTRAINT blocks_pkey PRIMARY KEY (height);


--
-- TOC entry 3215 (class 2606 OID 58502)
-- Name: boxes boxes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT boxes_pkey PRIMARY KEY (id);


--
-- TOC entry 3232 (class 2606 OID 58536)
-- Name: data_inputs data_inputs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.data_inputs
    ADD CONSTRAINT data_inputs_pkey PRIMARY KEY (id);


--
-- TOC entry 3230 (class 2606 OID 58529)
-- Name: inputs inputs_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inputs
    ADD CONSTRAINT inputs_pkey PRIMARY KEY (id);


--
-- TOC entry 3241 (class 2606 OID 58552)
-- Name: token_in_box token_in_box_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_in_box
    ADD CONSTRAINT token_in_box_pkey PRIMARY KEY (id);


--
-- TOC entry 3236 (class 2606 OID 58545)
-- Name: tokens tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens_pkey PRIMARY KEY (id);


--
-- TOC entry 3227 (class 2606 OID 58520)
-- Name: transactions transactions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);


--
-- TOC entry 3216 (class 1259 OID 169015)
-- Name: fki_fk_boxes_created; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_fk_boxes_created ON public.boxes USING btree (created);


--
-- TOC entry 3217 (class 1259 OID 169021)
-- Name: fki_fk_boxes_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_fk_boxes_spent ON public.boxes USING btree (spent);


--
-- TOC entry 3228 (class 1259 OID 2871134)
-- Name: fki_inputs__transactions_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_inputs__transactions_fk ON public.inputs USING btree (transaction_id);


--
-- TOC entry 3237 (class 1259 OID 2871152)
-- Name: fki_token_in_box__boxes_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_token_in_box__boxes_fk ON public.token_in_box USING btree (box_id);


--
-- TOC entry 3233 (class 1259 OID 2871140)
-- Name: fki_tokens__boxes_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_tokens__boxes_fk ON public.tokens USING btree (issuance_box);


--
-- TOC entry 3225 (class 1259 OID 2871146)
-- Name: fki_transactions__blocks_fk; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX fki_transactions__blocks_fk ON public.transactions USING btree (height);


--
-- TOC entry 3223 (class 1259 OID 1352195)
-- Name: idx_addresses_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_address ON public.addresses USING hash (address);


--
-- TOC entry 3224 (class 1259 OID 1352194)
-- Name: idx_addresses_ergotree; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_ergotree ON public.addresses USING hash (ergotree);


--
-- TOC entry 3213 (class 1259 OID 1452232)
-- Name: idx_blocks_timestamp; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_blocks_timestamp ON public.blocks USING btree ("timestamp");


--
-- TOC entry 3218 (class 1259 OID 169009)
-- Name: idx_boxes_address_created_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_boxes_address_created_spent ON public.boxes USING btree (address, created, spent);


--
-- TOC entry 3219 (class 1259 OID 169008)
-- Name: idx_boxes_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_boxes_box_id ON public.boxes USING btree (box_id);


--
-- TOC entry 3220 (class 1259 OID 1448507)
-- Name: idx_id_created_spent; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_id_created_spent ON public.boxes USING btree (id, created, spent);


--
-- TOC entry 3238 (class 1259 OID 1246542)
-- Name: idx_token_in_box_box_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_token_in_box_box_id ON public.token_in_box USING btree (box_id);


--
-- TOC entry 3239 (class 1259 OID 1251133)
-- Name: idx_token_in_box_token_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_token_in_box_token_id ON public.token_in_box USING btree (token_id) INCLUDE (box_id);


--
-- TOC entry 3234 (class 1259 OID 169022)
-- Name: idz_tokens_token_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idz_tokens_token_id ON public.tokens USING btree (token_id);


--
-- TOC entry 3242 (class 2606 OID 169010)
-- Name: boxes fk_boxes_created; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT fk_boxes_created FOREIGN KEY (created) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- TOC entry 3243 (class 2606 OID 169016)
-- Name: boxes fk_boxes_spent; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.boxes
    ADD CONSTRAINT fk_boxes_spent FOREIGN KEY (spent) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE SET NULL NOT VALID;


--
-- TOC entry 3245 (class 2606 OID 2871129)
-- Name: inputs inputs__transactions_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inputs
    ADD CONSTRAINT inputs__transactions_fk FOREIGN KEY (transaction_id) REFERENCES public.transactions(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- TOC entry 3247 (class 2606 OID 2871147)
-- Name: token_in_box token_in_box__boxes_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_in_box
    ADD CONSTRAINT token_in_box__boxes_fk FOREIGN KEY (box_id) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- TOC entry 3246 (class 2606 OID 2871135)
-- Name: tokens tokens__boxes_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens__boxes_fk FOREIGN KEY (issuance_box) REFERENCES public.boxes(id) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


--
-- TOC entry 3244 (class 2606 OID 2871141)
-- Name: transactions transactions__blocks_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions__blocks_fk FOREIGN KEY (height) REFERENCES public.blocks(height) ON UPDATE CASCADE ON DELETE CASCADE NOT VALID;


-- Completed on 2023-08-14 14:20:55 CEST

--
-- PostgreSQL database dump complete
--
