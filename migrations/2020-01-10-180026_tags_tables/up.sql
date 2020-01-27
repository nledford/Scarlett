-- Add tags table
-- Each row simply holds a single tag which must be lowercase
create table tags
(
    id       serial       not null
        constraint tags_pk primary key,
    tag_name varchar(100) not null
        constraint lowercase_tag_name
            check ( tag_name = lower(tag_name) )
);

-- ensure there cannot be duplicate tag names
create unique index idx_lower_tag_name_unique
    on tags (lower(tag_name));

-- Add `photo_tag` junction table
create table photo_tag
(
    id       serial not null
        constraint photo_tag_pk primary key,
    photo_id int    not null,
    tag_id   int    not null,
    constraint photo_tag_photos_fk foreign key (photo_id) references photos (id),
    constraint photo_tag_tags_fk foreign key (tag_id) references tags (id)
);

-- ensure there cannot be duplicate photo/tag combinations
create unique index idx_unique_photo_tag_combo
    on photo_tag (photo_id, tag_id);
