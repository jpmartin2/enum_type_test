CREATE TYPE foo as ENUM (
  'value1', 'value2', 'value3'
);

CREATE TABLE test (
  id SERIAL PRIMARY KEY,

  f foo
);
