CREATE TABLE IF NOT EXISTS "worlds" (
                                        "world_id" BigInt PRIMARY KEY,
                                        "name" TEXT NOT NULL UNIQUE,
                                        "data_center_id" BigInt NOT NULL,
                                        CONSTRAINT fk_data_center FOREIGN KEY(data_center_id) REFERENCES data_centers(id)
);
