-- Write your migration script here
create table if not exists first_table (
  id bigserial primary key,
  text_value text not null,
  number_value integer not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now(),
  deleted_at timestamp null default null
)