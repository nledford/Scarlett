-- displays metadata about tags
create or replace view tag_stats as
select tag_name,
       photos_with_tag,
       (photos_with_tag::decimal / photos_with_tags::decimal) * 100                  percentage_with_tag,
       (photos_with_tag::decimal / (select count(*)::decimal from photos_all)) * 100 percentage_total
from (select t.tag_name,
             (select count(pt.photo_id)
              from tags t2
                       left join photo_tag pt on t2.id = pt.tag_id
              where t2.id = t.id)                                      photos_with_tag,
             (select count(distinct photo_id)
              from tags t3
                       inner join photo_tag pt2 on t3.id = pt2.tag_id) photos_with_tags
      from tags t) s
order by photos_with_tag desc, tag_name;
