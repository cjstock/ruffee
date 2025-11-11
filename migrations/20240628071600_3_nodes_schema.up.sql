-- This migration creates the tables for the full data model based on the README.md.

-- Base entities
CREATE TABLE origin (
    origin_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('origin');

CREATE TABLE region (
    region_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    description TEXT,
    origin_id UUID REFERENCES origin(origin_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('region');

CREATE TABLE variety (
    variety_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL UNIQUE,
    species TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('variety');

CREATE TABLE processing_method (
    processing_method_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('processing_method');

CREATE TABLE roast_machine (
    roast_machine_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    manufacturer TEXT,
    url TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_machine');

CREATE TABLE ingredient (
    ingredient_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL UNIQUE,
    url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('ingredient');

CREATE TABLE roastery (
    roastery_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    location TEXT,
    url TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roastery');

CREATE TABLE roaster (
    roaster_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    url TEXT,
    description TEXT,
    roastery_id UUID REFERENCES roastery(roastery_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roaster');

CREATE TABLE mill (
    mill_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    location TEXT,
    url TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('mill');

CREATE TABLE miller (
    miller_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    url TEXT,
    description TEXT,
    mill_id UUID REFERENCES mill(mill_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('miller');

CREATE TABLE farm (
    farm_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    location TEXT,
    url TEXT,
    description TEXT,
    region_id UUID REFERENCES region(region_id),
    altitude TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('farm');

CREATE TABLE farmer (
    farmer_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    url TEXT,
    description TEXT,
    farm_id UUID REFERENCES farm(farm_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('farmer');

CREATE TABLE seller (
    seller_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    location TEXT,
    url TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('seller');

-- Events and Recipes
CREATE TABLE grow_event (
    grow_event_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    timestamp TIMESTAMPTZ,
    farm_id UUID REFERENCES farm(farm_id),
    farmer_id UUID REFERENCES farmer(farmer_id),
    variety_id UUID REFERENCES variety(variety_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('grow_event');

CREATE TABLE mill_event (
    mill_event_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    timestamp TIMESTAMPTZ,
    mill_id UUID REFERENCES mill(mill_id),
    miller_id UUID REFERENCES miller(miller_id),
    processing_method_id UUID REFERENCES processing_method(processing_method_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('mill_event');

CREATE TABLE roast_recipe (
    roast_recipe_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_recipe');

CREATE TABLE roast_recipe_step (
    roast_recipe_step_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    roast_recipe_id UUID NOT NULL REFERENCES roast_recipe(roast_recipe_id),
    name TEXT NOT NULL,
    duration_seconds INTEGER,
    temperature_celsius INTEGER,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_recipe_step');

CREATE TABLE roast_profile (
    roast_profile_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    description TEXT,
    roast_recipe_id UUID REFERENCES roast_recipe(roast_recipe_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_profile');

CREATE TABLE roast_event (
    roast_event_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    timestamp TIMESTAMPTZ,
    duration_seconds INTEGER,
    roaster_id UUID REFERENCES roaster(roaster_id),
    roastery_id UUID REFERENCES roastery(roastery_id),
    roast_machine_id UUID REFERENCES roast_machine(roast_machine_id),
    roast_profile_id UUID REFERENCES roast_profile(roast_profile_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_event');

CREATE TABLE roast_stage (
    roast_stage_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    timestamp TIMESTAMPTZ,
    duration_seconds INTEGER,
    temperature_celsius INTEGER,
    roast_event_id UUID NOT NULL REFERENCES roast_event(roast_event_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('roast_stage');

CREATE TABLE coffee_bean (
    coffee_bean_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    grow_event_id UUID REFERENCES grow_event(grow_event_id),
    mill_event_id UUID REFERENCES mill_event(mill_event_id),
    roast_event_id UUID REFERENCES roast_event(roast_event_id), -- Can be null if it's a green bean
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('coffee_bean');

CREATE TABLE blend (
    blend_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('blend');

CREATE TABLE recipe (
    recipe_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('recipe');

CREATE TABLE recipe_step (
    recipe_step_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    name TEXT NOT NULL,
    duration_seconds INTEGER,
    description TEXT,
    recipe_id UUID NOT NULL REFERENCES recipe(recipe_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('recipe_step');

CREATE TABLE purchase (
    purchase_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    seller_id UUID NOT NULL REFERENCES seller(seller_id),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('purchase');

CREATE TABLE taste (
    taste_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    timestamp TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT trigger_updated_at('taste');

-- Junction tables for many-to-many relationships
CREATE TABLE blend_coffee (
    blend_id UUID NOT NULL REFERENCES blend(blend_id),
    coffee_bean_id UUID NOT NULL REFERENCES coffee_bean(coffee_bean_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (blend_id, coffee_bean_id)
);
SELECT trigger_updated_at('blend_coffee');

CREATE TABLE recipe_ingredient (
    recipe_id UUID NOT NULL REFERENCES recipe(recipe_id),
    ingredient_id UUID NOT NULL REFERENCES ingredient(ingredient_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (recipe_id, ingredient_id)
);
SELECT trigger_updated_at('recipe_ingredient');

CREATE TABLE recipe_step_ingredient (
    recipe_step_id UUID NOT NULL REFERENCES recipe_step(recipe_step_id),
    ingredient_id UUID NOT NULL REFERENCES ingredient(ingredient_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (recipe_step_id, ingredient_id)
);
SELECT trigger_updated_at('recipe_step_ingredient');

CREATE TABLE roast_event_input_bean (
    roast_event_id UUID NOT NULL REFERENCES roast_event(roast_event_id),
    coffee_bean_id UUID NOT NULL REFERENCES coffee_bean(coffee_bean_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (roast_event_id, coffee_bean_id)
);
SELECT trigger_updated_at('roast_event_input_bean');
