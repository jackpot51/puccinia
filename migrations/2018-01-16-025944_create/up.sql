create table positions (
    id integer not null primary key,
    name text not null,
    units text not null,
    price text not null
);

create table transactions (
  id integer not null primary key,
  name text not null,
  amount text not null
);
