-- create `photos_all` view
-- This view will aggregate data from junction tables as text arrays using left joins
-- The process for constucting the query can be found from this StackOverflow answer: https://dba.stackexchange.com/a/173879

create view photos_all as
select id,
       file_path,
       file_name,
       file_hash,
       rating,
       date_created,
       date_updated,
       original_width,
       original_height,
       rotation,
       ineligible_for_wallpaper,
       anonymous_entities,
       e.entities,
       t.tags,
       w.wallpapers
from photos p
         LEFT JOIN (
    select pe.photo_id as id, array_agg(e.entity_name) as entities
    from photo_entity pe
             JOIN entity e on pe.entity_id = e.id
    group by pe.id) e using (id)
         LEFT JOIN (
    SELECT pt.photo_id as id, array_agg(t.tag_name) as tags
    FROM photo_tag pt
             JOIN tags t on pt.tag_id = t.id
    GROUP BY pt.id
) t using (id)
         LEFT JOIN (
    SELECT pw.photo_id as id, array_agg(ws.name) as wallpapers
    FROM photo_wallpaper pw
             JOIN wallpaper_sizes ws on pw.wallpaper_size_id = ws.id
    GROUP BY pw.id
) w using (id);