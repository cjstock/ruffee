create table coffee(
    coffee_id uuid primary key default uuid_generate_v1mc(),
    title text not null,
    description text,
    altitude integer,
    country text,
    region text,
    farm text,
    farmer text,
    variety text,
    process text,
    grade text,
    roast_level text,
    tasting_notes text[],
    recommended_brew_methods text[]
)
