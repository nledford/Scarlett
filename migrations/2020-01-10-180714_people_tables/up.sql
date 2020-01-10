-- Add entity table
create table entity
(
    id          serial       not null
        constraint entity_pk primary key,
    entity_name varchar(250) not null
);

-- add `photo_entity` junction table
create table photo_entity
(
    id        serial not null
        constraint photo_entity_pk primary key,
    photo_id  int    not null,
    entity_id int    not null,
    constraint photo_entity_photos_fk foreign key (photo_id) references photos (id),
    constraint photo_entity_entity_fk foreign key (entity_id) references entity (id)
);
