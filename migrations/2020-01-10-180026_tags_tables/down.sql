-- drop indexes
drop index idx_lower_tag_name_unique;
drop index idx_unique_photo_tag_combo;

-- Drop junction table
drop table if exists photo_tag;

-- Drop tags table
drop table if exists tags;