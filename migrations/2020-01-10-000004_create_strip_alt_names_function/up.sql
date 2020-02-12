create or replace function strip_alt_names(name text) returns text as
$stripped_text$
declare
    stripped_name text;
begin
    select case
               when name like '%[%]%' then trim(split_part(name, '[', 1))
               when name like '%(%)%' then trim(split_part(name, '(', 1))
               when name ~ '[A-Za-z]+[ ]?[A-Za-z]*[ ]\d'
                   then trim(regexp_replace(name, '[ ]\d+', ''))
               else trim(name) end
    into stripped_name;
    return stripped_name;
end ;
$stripped_text$ language plpgsql;
