-- Run this on an existing database that was created with init.sql
-- This marks existing migrations as applied so SeaORM won't try to re-run them
--
-- After running this, only m000012_create_mempool_tables will be pending

CREATE TABLE IF NOT EXISTS seaql_migrations (
    version VARCHAR NOT NULL PRIMARY KEY,
    applied_at BIGINT NOT NULL
);

INSERT INTO seaql_migrations (version, applied_at) VALUES
('m000001_create_blocks', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000002_create_addresses', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000003_create_transactions', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000004_create_boxes', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000005_create_tokens', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000006_create_inputs', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000007_create_token_in_box', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000008_create_asset_in_box', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000009_create_wrapped', EXTRACT(EPOCH FROM NOW())::BIGINT),
('m000010_create_functions', EXTRACT(EPOCH FROM NOW())::BIGINT)
ON CONFLICT (version) DO NOTHING;
