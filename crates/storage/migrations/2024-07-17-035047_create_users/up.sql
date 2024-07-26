-- Your SQL goes here

CREATE TABLE "users" (
	id SERIAL PRIMARY KEY NOT NULL,
	username VARCHAR NOT NULL,
	email VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	created_at TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email" ON "users"("email");