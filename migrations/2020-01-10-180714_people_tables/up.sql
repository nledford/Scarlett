-- instagram username regex source: https://regexr.com/3cg7r
-- twitter username regex source: https://stackoverflow.com/a/8650024

-- Add entity table
create table if not exists entity
(
    id                 serial             not null
        constraint entity_pk primary key,
    entity_name        varchar(250)       not null,
    alternate_names    text[],
    instagram_username varchar(30)
        constraint proper_instagram_username
            check (instagram_username ~* '/^(?!.*\.\.)(?!.*\.$)[^\W][\w.]{0,29}$/igm'),
    twitter_username   varchar(15)
        constraint proper_twitter_username
            check (twitter_username ~* '/^@?(\w){1,15}$/igm' ),
    favorite           bool default false not null,
    -- just in case...
    profile_photo_id   int
);

-- ensure there cannot be any duplicate entities
create unique index idx_unique_entities
    on entity (lower(entity_name));

-- create trigram index for text searching
create index idx_entity_name_search on entity using gin (entity_name gin_trgm_ops);

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

create index idx_photo_id_entity_id on photo_entity (photo_id, entity_id);
create index idx_entity_id_photo_id on photo_entity (entity_id, photo_id);

-- ensure there cannot be any photo/entity duplicates
create unique index idx_unique_photo_entity_combo
    on photo_entity (photo_id, entity_id);
