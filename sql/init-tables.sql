CREATE TABLE IF NOT EXISTS "quotes"(
                                       "id" SERIAL,
                                       "date" DATE NOT NULL,
                                       "sign" VARCHAR(20) NOT NULL,
                                       "quote" TEXT NOT NULL
);