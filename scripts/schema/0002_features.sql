-- Harness durable layer schema v2
-- Adds feature_list.json mirror columns to stories so `feature sync` can store
-- the full feature record, not just title/lane/status.

ALTER TABLE stories ADD COLUMN area TEXT;
ALTER TABLE stories ADD COLUMN priority INTEGER;
ALTER TABLE stories ADD COLUMN user_visible_behavior TEXT;
ALTER TABLE stories ADD COLUMN evidence TEXT;
