-- add `photo_stats` view
-- this view simply groups photos by their rating and sums them

CREATE OR REPLACE VIEW photos_stats AS
SELECT UNRATED,
       (UNRATED / TOTAL_KEPT::decimal) * 100              UNRATED_PERCENT,
       HIDDEN,
       (HIDDEN / TOTAL_KEPT::decimal) * 100               HIDDEN_PERCENT,
       NEUTRAL,
       (NEUTRAL / TOTAL_KEPT::decimal) * 100              NEUTRAL_PERCENT,
       WALLPAPER_CANDIDATES,
       (WALLPAPER_CANDIDATES / TOTAL_KEPT::decimal) * 100 WC_PERCENT,
       FAVORITES,
       (FAVORITES / TOTAL_KEPT::decimal) * 100            FAVORITES_PERCENT,
       WITH_ENTITIES,
       (WITH_ENTITIES / TOTAL_KEPT::decimal) * 100        WITH_ENTITIES_PERCENT,
       WITH_TAGS,
       (WITH_TAGS / TOTAL_KEPT::decimal) * 100            WITH_TAGS_PERCENT,
       WITH_WALLPAPER,
       (WITH_WALLPAPER / TOTAL_KEPT::decimal) * 100       WITH_WALLPAPER_PERCENT,
       TOTAL_KEPT,
       (TOTAL_KEPT / TOTAL::decimal) * 100                KEPT_PERCENT,
       PENDING_DELETE,
       (PENDING_DELETE / TOTAL::decimal) * 100            PENDING_DELETE_PERCENT,
       TOTAL
FROM (
         SELECT (SELECT COUNT(*)
                 from photos
                 where rating = 0)  UNRATED,
                (SELECT COUNT(*)
                 from photos
                 where rating = 1)  PENDING_DELETE,
                (SELECT COUNT(*)
                 from photos
                 where rating = 2)  HIDDEN,
                (SELECT COUNT(*)
                 from photos
                 where rating = 3)  NEUTRAL,
                (SELECT COUNT(*)
                 from photos
                 where rating = 4)  WALLPAPER_CANDIDATES,
                (SELECT COUNT(*)
                 from photos
                 where rating = 5)  FAVORITES,
                (SELECT COUNT(DISTINCT pe.photo_id)
                 FROM photos p
                          INNER JOIN photo_entity pe on p.id = pe.photo_id
                 WHERE rating <> 1) WITH_ENTITIES,
                (SELECT COUNT(DISTINCT pt.photo_id)
                 FROM photos p
                          inner join photo_tag pt on p.id = pt.photo_id
                 WHERE rating <> 1) WITH_TAGS,
                (SELECT COUNT(DISTINCT pw.photo_id)
                 FROM photos p
                          INNER JOIN photo_wallpaper pw on p.id = pw.photo_id
                 WHERE rating <> 1) WITH_WALLPAPER,
                (SELECT COUNT(*)
                 from photos
                 where rating <> 1) TOTAL_KEPT,
                (SELECT COUNT(*)
                 from photos)       TOTAL) s;
