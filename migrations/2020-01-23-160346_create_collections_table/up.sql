-- create the collections table
-- this will act as a "smart collections" table. Rather than manually specifying which pictures belong to a collection,
-- we'll store a partial WHERE clause that describes what images we want to retrieve for a specific collection

-- ideally I'd use a query builder on the client application and store a json/xml with the where clause but this will
-- at least get me started in building smart collections

create table collections
(
    id     serial        not null
        constraint collections_pk
            primary key,
    name   varchar(50)   not null,
    folder varchar(50) default null,
    query  varchar(1000) not null
);

create index idx_collections_name on collections (name);
create index idx_collections_folder on collections (folder);
create index idx_collections_folder_name on collections (folder, name);
create index idx_collections_name_query on collections (name, query);
create index idx_collections_name_search on collections using gin (name gin_trgm_ops);
create index idx_collections_query_search on collections using gin (query gin_trgm_ops);

-- ensure each collection has a unique name
create unique index idx_unique_collection_names
    on collections (lower(name));

-- ensure each collection has a unique query
create unique index idx_unique_collection_queries
    on collections (lower(query));

-- ensure there cannot be any duplicate collections
create unique index idx_unique_collections
    on collections (name, query);

------------------------------------------------------------------------------------------------------------------------
-- default collections
------------------------------------------------------------------------------------------------------------------------

-- photos that have never been viewed
insert into collections (name, folder, query)
VALUES ('Never viewed', 'Last Viewed', 'last_viewed is null');

-- photos that have not been view in the past thirty days
insert into collections (name, folder, query)
VALUES ('Not in past thirty days',
        'Last Viewed',
        'last_viewed is null or last_viewed <= current_timestamp - interval ''30 days''');

-- photos that do not have entities and are not marked as being anonymous
insert into collections(name, folder, query)
VALUES ('Missing Entities',
        'Metadata',
        'anonymous_entities is false and (entities is null or cardinality(entities) = 0)');

-- photos that do not have tags
insert into collections (name, folder, query)
values ('No Tags',
        'Metadata',
        'tags is null or cardinality(tags) <= 0');

-- all favorites
insert into collections (name, folder, query)
VALUES ('All Favorites', 'Favorites', 'rating in (4, 5)');