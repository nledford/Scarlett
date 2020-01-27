-- The `entity_full` view combines the `sorted_entity` view with statistics about photos and wallpapers

create or replace view entity_full as
select id,
       entity_name,
       sort_name,
       alternate_names,
       instagram_username,
       twitter_username,
       favorite,
       profile_photo_id,
       p.num_photos,
       (p.num_photos::decimal / (SELECT with_entities::decimal
                                 FROM photos_stats)) * 100     PHOTOS_PERCENT,
       w.num_wallpapers,
       (w.num_wallpapers::decimal / (SELECT with_wallpaper::decimal
                                     FROM photos_stats)) * 100 WALLPAPER_PERCENT
from sorted_entity se
         LEFT JOIN (
    select pe.entity_id                  as id,
           NULLIF(count(pe.photo_id), 0) as num_photos
    from photo_entity pe
             JOIN photos p on pe.photo_id = p.id
    group by pe.entity_id
) p using (id)
         LEFT JOIN (
    select pe2.entity_id                          as id,
           NULLIF(count(pw.wallpaper_size_id), 0) as num_wallpapers
    from photo_entity pe2
             inner join photos p2 on pe2.photo_id = p2.id
             left join photo_wallpaper pw on p2.id = pw.photo_id
    group by pe2.entity_id
) w using (id);