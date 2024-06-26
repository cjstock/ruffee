create table coffee(
    coffee_id uuid primary key default uuid_generatev1mc(),
    title text not null
    description text
    altitude integer
    created_at timestamptz not null default now(),
    updated_at timestamptz
    );

select trigger_updated_at('"coffee"');
