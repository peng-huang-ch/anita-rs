-- Your SQL goes here

-- CreateTable
CREATE TABLE IF NOT EXISTS "keys" (
    id SERIAL PRIMARY KEY,
    secret VARCHAR NOT NULL,
    suffix VARCHAR NOT NULL,
    used_at TIMESTAMP(3),
    created_at TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP
);

-- CreateIndex
CREATE UNIQUE INDEX "keys_secret" ON "keys"("secret");
CREATE INDEX "keys_secret_suffix_idx" ON "keys"("suffix");
