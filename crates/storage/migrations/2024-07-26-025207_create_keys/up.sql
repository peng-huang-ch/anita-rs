-- Your SQL goes here

-- CreateTable
CREATE TABLE IF NOT EXISTS "keys" (
    id SERIAL PRIMARY KEY,
    chain VARCHAR NOT NULL,
    secret BYTEA NOT NULL,
    pubkey VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    suffix VARCHAR NOT NULL,
    used_at TIMESTAMP(3),
    created_at TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP
);

-- CreateIndex
CREATE UNIQUE INDEX "keys_secret" ON "keys"("secret");
CREATE INDEX "keys_pubkey_idx" ON "keys"("pubkey");
CREATE INDEX "keys_chain_suffix_idx" ON "keys"("chain","suffix");
