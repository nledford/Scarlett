-- delete tree view
drop view directory_tree;

-- delete the get tree function
drop function if exists get_tree(paths json);

