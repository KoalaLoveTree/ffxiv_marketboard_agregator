CREATE TABLE IF NOT EXISTS data_centers
(
    `id`     BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name`   TEXT            NOT NULL UNIQUE,
    `region` TEXT            NOT NULL,
    PRIMARY KEY (id)
);
