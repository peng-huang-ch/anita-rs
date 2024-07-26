-- This file should undo anything in `up.sql`

-- DropIndex
DROP INDEX IF EXISTS "keys_secret";
DROP INDEX IF EXISTS "keys_pubkey_idx";
DROP INDEX IF EXISTS "keys_chain_suffix_idx";

-- DropTable
DROP TABLE IF EXISTS "keys";
