-- This file should undo anything in `up.sql`

-- DropIndex
DROP INDEX IF EXISTS "users_email";

-- DropTable
DROP TABLE IF EXISTS "users";