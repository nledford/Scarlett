-- drop indexes
drop index idx_unique_entities;
drop index idx_unique_photo_entity_combo;

-- delete junction table first
drop table if exists photo_entity;

-- drop entities table
drop table if exists entity;