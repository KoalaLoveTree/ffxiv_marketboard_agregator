CREATE TABLE IF NOT EXISTS "items_trade_volumes" (
                                                     "id" BIGSERIAL PRIMARY KEY,
                                                     "item_id" BigInt NOT NULL,
                                                     "world_id" BigInt NOT NULL,
                                                     "cheapest_world_id" BigInt NOT NULL,
                                                     "sale_score" decimal NOT NULL,
                                                     "price_diff_score" decimal NOT NULL,
                                                     CONSTRAINT fk_item FOREIGN KEY(item_id) REFERENCES items(item_id),
                                                     CONSTRAINT fk_world FOREIGN KEY(world_id) REFERENCES worlds(world_id),
                                                     CONSTRAINT fk_cheapest_world FOREIGN KEY(cheapest_world_id) REFERENCES worlds(world_id)
);
