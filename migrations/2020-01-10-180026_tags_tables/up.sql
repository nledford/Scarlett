-- Add tags table
-- Each row simply holds a single tag
create table tags
(
    id       serial       not null
        constraint tags_pk primary key,
    tag_name varchar(100) not null
);

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
