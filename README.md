# SQL GENERATOR

Sql create,update and insert generator;

## Example

cargo run -- -t users -f id:integer:primarykey -f email:varchar:notnull:unique -f age:integer

## Output

CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  age INTEGER
);

INSERT INTO users (
  id,
  email,
  age
)
VALUES (
  'id',
  'email',
  'age'
);

UPDATE users SET
  email = <value>,
  age = <value>
WHERE id = <value>;
