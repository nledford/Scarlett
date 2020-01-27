-- create the collections table
-- this will act as a "smart collections" table. Rather than manually specifying which pictures belong to a collection,
-- we'll store a partial WHERE clause that describes what images we want to retrieve for a specific collection

-- ideally I'd use a query builder on the client application and store a json/xml with the where clause but this will
-- at least get me started in building smart collections

create table collections
(
    id    serial     not null
        constraint collections_pk
            primary key,
    name  varchar(50)   not null,
    query varchar(1000) not null
);