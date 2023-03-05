CREATE TABLE IF NOT EXISTS items_trade_volumes
(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `item_id`           BIGINT UNSIGNED NOT NULL,
    `world_id`          BIGINT UNSIGNED NOT NULL,
    `cheapest_world_id` BIGINT UNSIGNED NOT NULL,
    `sale_score`        DOUBLE NOT NULL,
    `price_diff_score`  DOUBLE NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (item_id) REFERENCES items (item_id),
    FOREIGN KEY (world_id) REFERENCES worlds (world_id),
   FOREIGN KEY (cheapest_world_id) REFERENCES worlds (world_id)
);
