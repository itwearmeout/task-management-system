create table "user_tasks"
(
    user_id uuid not null references users(user_id) on delete cascade,
    task_id uuid primary key default uuid_generate_v1mc(),
    task_subject text not null,
    due_at timestamptz not null,
    is_complete boolean default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);