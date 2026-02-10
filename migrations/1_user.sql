create extension if not exists "uuid-ossp";

create collation case_insensitive (
      provider = icu,
      locale = 'und-u-ks-level2',
      deterministic = false
);

create table "users"
(
    user_id uuid primary key default uuid_generate_v1mc(),
    username text collate "case_insensitive" unique not null,
    email text collate "case_insensitive" unique not null,
    angkatan int not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
