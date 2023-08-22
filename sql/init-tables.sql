CREATE TABLE IF NOT EXISTS "quotes"(
                                       "id" SERIAL,
                                       "date" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                       "sign" VARCHAR(20) NOT NULL,
                                       "quote" TEXT NOT NULL
);