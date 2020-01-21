-- first create the get tree function using javascript rather than plsql
create or replace function get_tree(paths json) returns json as
$$
    let root = []

    paths.forEach((path) => {
        let pathParts = path.split('/')
        pathParts.pop()

        // Initialize current level to root
        let currentLevel = root

        let currentPath = ''
        let currentId = 'root';
        pathParts.forEach((part) => {
            currentPath = '${currentPath}/${part}'

            const existingPath = currentLevel.find((o) => o.name === part)

            if (existingPath) {
                currentLevel = existingPath.children
            } else {
                // generates a random string in javascript, not guaranteed to be unique
                // SOURCE: https://gist.github.com/6174/6062387
                const id =  Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15)

                let newPart = {
                    id: id,
                    parentId: currentId,
                    name: part,
                    children: [],
                }

                currentId = newPart.id

                currentLevel.push(newPart);
                currentLevel = newPart.children;
            }
        })
        currentPath = '';
    })

    return JSON.stringify(root);
$$ LANGUAGE plv8 immutable
                 strict;

-- then create the tree view

create or replace view directory_tree as
with data as (
    select array_to_json(array_agg(folder)) as data
    from (select folder
          from photos_all
          group by folder
          order by lower(folder)) s
)
select jsonb_pretty(get_tree(data)::jsonb) directory_tree
from data;

select * from directory_tree;