-- View adds a column that reconstructs the `entity_name` column into last name, first name

create or replace view sorted_entity as
select id,
       entity_name,
       case
           when cardinality(name_arr) = 1 then name_arr[1]
           else
                       name_arr[array_upper(name_arr, 1)]
                       || ', '
                   || replace(entity_name, name_arr[array_upper(name_arr, 1)], '') end sort_name,
       alternate_names,
       instagram_username,
       twitter_username,
       favorite,
       profile_photo_id
from (select e.*,
             case
                 when entity_name like '%[%]%' then string_to_array(trim(split_part(entity_name, '[', 1)), ' ')
                 when entity_name like '%(%)%' then string_to_array(trim(split_part(entity_name, '(', 1)), ' ')
                 when entity_name ~ '[A-Za-z]+[ ]?[A-Za-z]*[ ]\d'
                     then string_to_array(trim(regexp_replace(entity_name, '[ ]\d+', '')), ' ')
                 else string_to_array(trim(entity_name), ' ') end name_arr
      from entity e) v
order by sort_name;