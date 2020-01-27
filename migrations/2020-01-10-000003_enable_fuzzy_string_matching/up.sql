-- Enables trigrams in Postgresql
-- allows for use of fuzzy string matching
-- SEE: https://www.freecodecamp.org/news/fuzzy-string-matching-with-postgresql/

CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- example for fuzzy string matching entities:
-- `select * from sorted_entity order by similarity(entity_name, 'M Ash') desc;`
