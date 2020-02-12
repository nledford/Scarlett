-- Create initial photos schema
-- This table will serve as the foundation. Other tables, such as tags, people, and wallpaper will link back to it via
-- junction tables

create table if not exists photos
(
    id                       serial                                      not null
        constraint photo_pk
            primary key,
    file_path                varchar(1000)                               not null,
    file_name                varchar(255) default '0'::character varying not null,
    file_hash                varchar(255) default '0'::character varying not null,
    rating                   integer      default 0                      not null
        constraint rating_range
            check ((rating >= 0) AND (rating <= 5)),
    date_created             timestamp    default CURRENT_TIMESTAMP      not null,
    date_updated             timestamp    default CURRENT_TIMESTAMP      not null,
    constraint valid_update_time
        check ( date_updated >= current_timestamp::date - interval '10 seconds' ),
    last_viewed              timestamp    default null
        constraint valid_last_viewed_time
            check ( last_viewed >= current_timestamp::date - interval '10 seconds' ),
    original_width           integer      default 0                      not null
        constraint valid_photo_width
            check ( original_width >= 0 ),
    original_height          integer      default 0                      not null
        constraint valid_photo_height
            check ( original_height >= 0 ),
    rotation                 integer      default 0                      not null
        constraint rotation_values
            check ( rotation = 0
                or rotation = 90
                or rotation = 180
                or rotation = 270 ),
    ineligible_for_wallpaper bool         default false                  not null,
    anonymous_entities       bool         default false                  not null
);

-- create indexes on date columns
create index idx_photos_date_created on photos (date_created);
create index idx_photos_date_updated on photos (date_updated);
create index idx_photos_last_viewed on photos (last_viewed);

-- create index for file name and file path
create index idx_photos_file_name on photos (file_name);
create index idx_photos_file_path on photos (file_path);

-- create index for rating
create index if not exists idx_photos_rating on photos (rating);

-- create text searching indexes
create index idx_photos_file_name_search on photos using gin (file_name gin_trgm_ops);
create index idx_photos_file_path_search on photos using gin (file_path gin_trgm_ops);