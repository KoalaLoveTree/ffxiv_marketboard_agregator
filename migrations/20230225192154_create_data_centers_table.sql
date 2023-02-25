CREATE TABLE IF NOT EXISTS "data_centers" (
                                              "id" BIGSERIAL PRIMARY KEY, "name" TEXT NOT NULL UNIQUE,
                                              "region" TEXT NOT NULL
);
