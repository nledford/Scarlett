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
