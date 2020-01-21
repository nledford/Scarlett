-- creates a base folder hierarchy view
-- breaks the folder column from the photos_all view down into a parent/child relationship
create or replace view folder_hierarchy as
select parent_folder, folder
from (select (array_remove(string_to_array(pa.folder, '/'), ''))[a.level - 1] parent_folder, a.folder, a.level
      from photos_all pa
               left join lateral unnest(array_remove(string_to_array(pa.folder, '/'), '')) with ordinality as a(folder, level)
                         on true) s
group by parent_folder, folder, level
order by level, lower(parent_folder) nulls first, lower(folder);
