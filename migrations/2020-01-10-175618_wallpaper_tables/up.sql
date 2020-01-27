-- Add `wallpaper_sizes` table
-- This table will hold a list of all wallpaper sizes
create table wallpaper_sizes
(
    id     serial      not null
        constraint wallpaper_sizes_pk
            primary key,
    name   varchar(30) not null,
    width  int         not null,
    height int         not null
);

-- Add 'photo_wallpaper` junction table
create table photo_wallpaper
(
    id                serial        not null
        constraint photo_wallpaper_pk primary key,
    photo_id          int           not null,
    wallpaper_size_id int           not null,
    file_path         varchar(1000) not null,
    constraint photo_wallpaper_photos_fk foreign key (photo_id) references photos (id),
    constraint photo_wallpaper_wallpaper_sizes_fk foreign key (wallpaper_size_id) references wallpaper_sizes (id)
);

-- Add initial values for `wallpaper_sizes` table
INSERT INTO wallpaper_sizes (name, width, height)
VALUES ('Full HD', 1920, 1080),
       -- Surface Pro 4
       ('4.99M2', 2736, 1824),
       -- 15" MacBook Pro Retina Display
       ('5.18MA', 2880, 1800),
       ('Ultra-Wide 4K (WQHD+)', 3840, 1600),
       ('4K', 3840, 2160),
       ('5K', 5120, 2880),
       -- Apple Display that you can't afford
       ('6K', 6016, 3384),
       ('8K', 7680, 4320),
       ('16K', 15360, 8640);

-- Add view for pre-formatted wallpaper size display names
create view wallpaper_display_names as
select id,
       name
           || ' ['
           || width
           || ' x '
           || height
           || ']' display_name
from wallpaper_sizes
