# Coffee Tracking Application - Database Schema README

This document outlines the requirements and design of the database schema for a coffee tracking application. The database is designed using PostgreSQL.

## Overview

The application is designed to track various aspects of coffee, from its origin (farm) to brewing, including green coffee beans, roasting processes, and brewing recipes.  It also includes features for identifying potential duplicate entries and managing user accounts.

## Key Features & Requirements:

1.  **User Accounts:**
    *   Users can register and log in.
    *   User data is stored securely (passwords are hashed).
    *   Users can be "soft-deleted" (marked as inactive) without deleting their associated data.
    *   `Users` table with `user_id` (UUID), `username`, `email`, `password_hash`, `registration_date`, `last_login`, and `is_active`.

2.  **Coffee Origins:**
    *   Track coffee farms (`Farmers` table): `farmer_id` (UUID), `name`, `location`, `website`, `description`, `is_enabled`.
    *   Track coffee varietals (`Varietals` table): `varietal_id` (UUID), `name`, `description`, `origin`, `website`, `is_enabled`.
    *   Track roasting companies (`Roasters` table): `roaster_id` (UUID), `name`, `location`, `website`, `description`, `logo_url`, `is_enabled`.
    *   Infer the likely origin (farmer) of a roasted coffee based on roasting events and dates, since precise batch tracking is not available.

3.  **Green Coffee Inventory:**
    *   Track unroasted green coffee beans (`GreenCoffees` table): `green_coffee_id` (UUID), `farmer_id`, `name`, `description`, `purchase_date`, `weight`, `cost_per_unit`, `is_enabled`.
    * Many-to-many relationship between `GreenCoffees` and `Varietals`

4.  **Roasting:**
    *   Track *types* of roasting machines (`RoastingMachineModels` table): `machine_model_id` (UUID), `manufacturer`, `model_name`, `description`, `website`, `is_enabled`.
    *   Define roasting recipes (`RoastingRecipes` table): `roasting_recipe_id` (UUID), `name`, `description`, `machine_model_id` (required), `roaster_id`, `is_enabled`.
        *   A "Generic" machine model is used for recipes not tied to a specific machine.
    *   Log individual roasting events (`RoastingLogs` table): `roasting_log_id` (UUID), `roasting_recipe_id`, `green_coffee_id`, `roast_date`, `start_time`, `end_time`, `first_crack_time`, `second_crack_time`, `roast_level` (integer, 1-100), `weight_before`, `weight_after`, `notes`.

5.  **Roasted Coffee:**
    *   Track roasted coffee beans (`Coffees` table): `coffee_id` (UUID), `name`, `roaster_id`, `farmer_id` (inferred), `roasting_log_id`, `roast_date`, `description`, `roast_level` (integer, 1-100), `image_url`, `is_enabled`.
    *   Many-to-many relationship between `Coffees` and `Varietals`.

6.  **Brewing Recipes:**
    *   Track brewing recipes (`Recipes` table): `recipe_id` (SERIAL), `name`, `brew_method`, `coffee_id`, `grind_size`, `coffee_amount`, `water_amount`, `water_temperature`, `brew_time`, `instructions`, `is_enabled`.

7.  **Tasting Notes:**
    *   Manage a list of unique tasting notes (`TastingNotes` table): `tasting_note_id` (SERIAL), `note`, `is_enabled`.
    *   Many-to-many relationship between `Coffees` and `TastingNotes` (`CoffeeTastingNotes` table).
    *   Many-to-many relationship between `GreenCoffees` and `TastingNotes` (`GreenCoffeeTastingNotes` table).

8.  **Processing Methods:**
    *   Manage a list of unique processing methods (`ProcessingMethods` table):  `processing_method_id` (SERIAL), `method_name`, `description`, `is_enabled`.
    *   Many-to-many relationship between `Coffees` and `ProcessingMethods` (`CoffeeProcessingMethods` table).
    *  Many-to-many relationship between `GreenCoffees` and `ProcessingMethods` (`GreenCoffeeProcessingMethods` table)

9.  **Duplicate Detection:**
    *   Flag potential duplicate entries for Roasters, Farmers, Varietals, and Coffees.
    *   Store potential duplicates in a `PotentialDuplicates` table: `duplicate_id` (SERIAL), `entity_type`, `entity_id_1`, `entity_id_2`, `match_score`, `match_reasons`, `status`, `created_at`, `resolved_at`, `resolved_by`.
    *   Provide an administrative interface for reviewing and resolving (confirming, rejecting, or merging) potential duplicates.

10. **Data Integrity and Optimization:**
    *   Use UUIDs for primary keys on most tables for global uniqueness.
    *   Use `SERIAL` (auto-incrementing integers) for `recipe_id`, `tasting_note_id`, and `duplicate_id`.
    *   Use appropriate data types (VARCHAR, TEXT, DATE, TIMESTAMP, REAL, INTEGER, BOOLEAN, UUID).
    *   Use `NOT NULL` constraints on required fields.
    *   Use `ON DELETE SET NULL` for foreign keys referencing `Users` to preserve data when a user is soft-deleted.
    *   Use a numerical scale (1-100) for `roast_level`.
    *   **Partial Indexes:** Partial indexes are used on the `is_enabled` column of relevant tables to optimize queries for enabled entities.
    * Each entity has an `is_enabled` flag for enabling/disabling by administrative review

## Table Summary:

| Table Name                | Primary Key       | Description                                                                         |
| ------------------------- | ----------------- | ----------------------------------------------------------------------------------- |
| `Users`                   | `user_id` (UUID)  | User accounts.                                                                    |
| `Roasters`                | `roaster_id` (UUID) | Roasting companies.                                                                |
| `Farmers`                 | `farmer_id` (UUID) | Coffee farms.                                                                      |
| `Varietals`               | `varietal_id` (UUID)| Coffee varietals.                                                                  |
| `RoastingMachineModels`   | `machine_model_id` (UUID) | Types of roasting machines.                                                      |
| `GreenCoffees`           | `green_coffee_id` (UUID) | Unroasted coffee beans.                                                           |
| `GreenCoffee_Varietals` | (composite key)   | Many-to-many relationship between `GreenCoffees` and `Varietals`.                       |
| `RoastingRecipes`          | `roasting_recipe_id` (UUID)| Roasting recipes/profiles.                                                        |
| `RoastingLogs`            | `roasting_log_id` (UUID) | Records of individual roasting events.                                              |
| `Coffees`                 | `coffee_id` (UUID) | Roasted coffee beans.                                                               |
| `Coffee_Varietals`      | (composite key)   | Many-to-many relationship between `Coffees` and `Varietals`.                         |
| `Recipes`                 | `recipe_id` (SERIAL) | Brewing recipes.                                                                  |
| `TastingNotes`             | `tasting_note_id` (SERIAL) | Unique tasting notes.                                                           |
| `CoffeeTastingNotes`      | (composite key)   | Many-to-many relationship between `Coffees` and `TastingNotes`.                       |
| `GreenCoffeeTastingNotes` | (composite key)    | Many-to-many relationship between `GreenCoffees` and `TastingNotes`.                  |
| `ProcessingMethods`       | `processing_method_id` (SERIAL) | Unique coffee processing methods.                                                  |
| `CoffeeProcessingMethods`  | (composite key)    | Many-to-many relationship between `Coffees` and `ProcessingMethods`.               |
| `GreenCoffeeProcessingMethods` | (composite key) | Many-to-many relationship between `GreenCoffees` and `ProcessingMethods`        |
| `PotentialDuplicates`     | `duplicate_id` (SERIAL) | Records of potential duplicate entries.                                              |

## Relationships:

*   Users 1:M Roasters (`ON DELETE SET NULL`)
*   Users 1:M Farmers (`ON DELETE SET NULL`)
*   Users 1:M Varietals (`ON DELETE SET NULL`)
*   Users 1:M GreenCoffees (`ON DELETE SET NULL`)
*   Users 1:M RoastingRecipes (`ON DELETE SET NULL`)
*   Users 1:M RoastingLogs (`ON DELETE SET NULL`)
*   Users 1:M Coffees (`ON DELETE SET NULL`)
*   Users 1:M Recipes (`ON DELETE SET NULL`)
*	Roasters 1:M RoastingRecipes
*   Farmers 1:M GreenCoffees
*   GreenCoffees M:N Varietals
*   RoastingMachineModels 1:M RoastingRecipes
*   RoastingRecipes 1:M RoastingLogs
*   RoastingLogs 1:1 Coffees
*   Coffees M:N Varietals
*   Coffees M:N TastingNotes
*	GreenCoffees M:N TastingNotes
*   Coffees 1:M Recipes
*   Coffees M:N ProcessingMethods
*  Green Coffees M:N ProcessingMethods

## Notes:

*   The `roast_level` field in `Coffees` and `RoastingLogs` uses an integer scale (1-100).
*   The `PotentialDuplicates` table is used to flag potential duplicate entries for administrative review.
*   The application uses inferred traceability to determine the likely origin of a coffee, as precise batch tracking is not available.
*   A "Generic" machine model is used in `RoastingMachineModels` to represent recipes not tied to a specific machine.
* This schema uses UUIDs for most primary keys, and relies on `ON DELETE SET NULL` for soft-deletes of users, as well as an `is_enabled` flag on many tables for administrative control.
* This schema utilizes join tables to represent many-to-many relationships

