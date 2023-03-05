CREATE TABLE IF NOT EXISTS worlds
(
    `world_id`       BIGINT UNSIGNED NOT NULL,
    `name`           TEXT            NOT NULL UNIQUE,
    `data_center_id` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (world_id),
    FOREIGN KEY (data_center_id) REFERENCES data_centers (id)
);
