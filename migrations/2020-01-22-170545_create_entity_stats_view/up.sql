create or replace view entity_stats as
select entity_name,
       photos_with_entity,
       (photos_with_entity::decimal / photos_with_entities::decimal) * 100              percentage_with_entity,
       (photos_with_entity::decimal / (select count(*)::decimal from photos_all)) * 100 percentage_total
from (select se.entity_name,
             se.sort_name,
             (select count(pe.photo_id)
              from sorted_entity se2
                       left join photo_entity pe on se2.id = pe.entity_id
              where se2.id = se.id)                                       photos_with_entity,
             (select count(distinct photo_id)
              from sorted_entity se3
                       inner join photo_entity p on se3.id = p.entity_id) photos_with_entities
      from sorted_entity se) s
order by photos_with_entity desc, sort_name;