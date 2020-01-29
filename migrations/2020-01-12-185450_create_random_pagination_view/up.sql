-- Create a materialized view to help view random sorting and pagination of photos
-- SOURCE: https://schinckel.net/2019/11/08/random%2C-fixed-ordering-and-pagination/

CREATE MATERIALIZED VIEW photo_ordering AS
SELECT photo.id             AS photo_id,
       row_number() over () AS position
FROM (
         SELECT id
         FROM photos
         ORDER BY gen_random_uuid()
     ) photo;

CREATE INDEX photo_ordering_id ON photo_ordering (photo_id);
CREATE INDEX photo_ordering_position ON photo_ordering (position);