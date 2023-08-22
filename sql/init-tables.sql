CREATE TABLE IF NOT EXISTS "quotes"(
                                       "id" SERIAL,
                                       "date" TIMESTAMP NOT NULL,
                                       "sign" VARCHAR(20) NOT NULL,
                                       "quote" TEXT NOT NULL
);