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
    original_width           integer      default 0                      not null,
    original_height          integer      default 0                      not null,
    rotation                 integer      default 0                      not null,
    ineligible_for_wallpaper bool         default false                  not null,
    anonymous_entities       bool         default false                  not null
);