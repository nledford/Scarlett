-- View adds a column that reconstructs the `entity_name` column into last name, first name

create view sorted_entity as
select id,
       entity_name,
       name_arr[array_upper(name_arr, 1)]
           || ', '
           || replace(entity_name, name_arr[array_upper(name_arr, 1)], '') sort_name,
       alternate_names,
       instagram_username,
       twitter_username,
       favorite,
       profile_photo_id
from (select e.*, string_to_array(e.entity_name, ' ') name_arr
      from entity e) v
order by sort_name;