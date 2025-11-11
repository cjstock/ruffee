# Bunnfn

# Overview
Bunnfn allows coffee drinkers, roasters, growers, and processors to track, search, discover, and learn coffee.

---

# Features

## Track Everything
Track ALL the coffee things in Bunnfn. Check out 

## Search for Anything
Bunnfn's fast global search allows you to instantly jump to any node on Bunnfn's sprawling information graph

## Discover something
Bunnfn's vast information graph likely contains information you've never even thought about before! To check out the kinds of information you can find in Bunnfn, check out the [Node](#nodes) section.

## Privacy Control
When users add information to Bunnfn, they control what is private or public. Private information can only be seen by the user that created the information

---

# Nodes
The nodes of Bunnfn's information graph


## Node Templates

Nodes will typically fall into a few categories, which will hint at the kind of information it will store.


### Person
- Name*
- Url
- Description

### Place
- Name*
- Url
- Location
- Description

### Event
- Name*
- Timestamp*
- Duration
- Description

---

The following are the actual Nodes in Bunnfn. More could be added over time.

Fields marked with `*` are required.

Fields marked with `+` are references to other table entries.

Fields marked with `++` could reference multiple entries to other tables.

### Taste (Event)
A user is tasting a coffee.

**Fields**  
- Timestamp*: When this tasting occurred

### Coffee Bean
A single-origin coffee.

**Fields**  
- Name*: The name of the coffee. Usually set by the roaster of the coffee
- Roast Event+: Information about when, where, and how this coffee was roasted
- Mill Event+: Information about when, where, and how this coffee was milled
- Grow Event+: Information about when, where, and how this coffee was grown

### Blend
A blend of multiple single-origin coffees

**Fields**  
- Name*: The name of the blend. Usually set by the roaster
- Coffees++: The coffees that make up this blend

### Recipe
A recipe used to create a coffee beverage.

**Fields**  
- Ingredients*++: The edible ingredients used in the recipe
- 

### Ingredient
An edible ingredient in a beverage.

- Name*: Name of the ingredient
- Url: Weblink to the ingredient

### Recipe Step
A step in a recipe.

- Name*: Name of the step
- Ingredients++: Ingredients used in this step
- Duration: Duration of this event
- Description: A more detailed description of the step

### Purchase (Event)
A coffee was purchased.

- Seller*+: Where the coffee was purchased

### Seller (Place)
A place where coffee is sold. Could be a physical location or just an online store.

### Roast (Event)
The event of roasting a batch of green coffee beans.
- Roaster+: The person who performed the roast.
- Roastery+: The place where the roast occurred.
- Roast Machine+: The machine used for roasting.
- Roast Profile+: The roast profile used.
- Green Beans+: The green coffee beans that were roasted.
- Timestamp*: When the roast started.
- Duration: How long the roast took.

### Roastery (Place)
A facility where coffee is roasted.
- Name*: The name of the roastery.
- Location: The physical address.
- Url: A website for the roastery.
- Description: A description of the roastery.

### Roaster (Person)
A person who roasts coffee.
- Name*: The name of the roaster.
- Employer+: The roastery they work for.
- Url: A personal website or social media link.
- Description: A bio for the roaster.

### Roast Profile
A set of parameters that guide a roast.
- Name*: The name of the profile.
- Roast Recipe+: The recipe this profile follows.
- Description: A description of the intended outcome.

### Roast Stage (Event)
A specific stage within a roast event (e.g., "First Crack", "Drying").
- Name*: The name of the stage.
- Roast Event+: The parent roast event.
- Timestamp*: When the stage began.
- Duration: How long the stage lasted.
- Temperature: The temperature at this stage.

### Roast Recipe
A sequence of steps for a roast.
- Name*: The name of the recipe.
- Steps++: The sequence of roast stages.
- Description: A description of the recipe.

### Roast Machine
A machine used for roasting coffee.
- Name*: The name or model of the machine.
- Manufacturer: The company that made the machine.
- Url: A link to the product page.
- Description: Details about the machine.

### Mill (Event)
The event of processing coffee cherries at a mill.
- Mill+: The place where the milling occurred.
- Miller+: The person who oversaw the milling.
- Processing Method+: The method used.
- Timestamp*: When the milling occurred.
- Input Cherries++: The coffee cherries that were processed.
- Output Green Beans++: The resulting green beans.

### Mill (Place)
A facility where coffee cherries are processed into green beans.
- Name*: The name of the mill.
- Location: The physical address.
- Url: A website for the mill.
- Description: A description of the mill.

### Miller (Person)
A person who operates a coffee mill.
- Name*: The name of the miller.
- Employer+: The mill they work for.
- Url: A personal website or social media link.
- Description: A bio for the miller.

### Processing Method
A specific technique for processing coffee cherries (e.g., Washed, Natural, Honey).
- Name*: The name of the method.
- Description: A detailed explanation of the method.

### Grow (Event)
The process of cultivating a crop of coffee.
- Farm+: The farm where the coffee was grown.
- Farmer+: The farmer who grew the coffee.
- Variety+: The coffee variety grown.
- Harvest Start*: The date the harvest began.
- Harvest End: The date the harvest ended.
- Output Cherries++: The harvested coffee cherries.

### Farm (Place)
A place where coffee is cultivated.
- Name*: The name of the farm.
- Origin+: The origin/region the farm is in.
- Altitude+: The altitude range of the farm.
- Location: Specific coordinates or address.
- Url: A website for the farm.
- Description: A description of the farm.

### Farmer (Person)
A person who cultivates coffee.
- Name*: The name of the farmer.
- Farm+: The farm they work on.
- Url: A personal website or social media link.
- Description: A bio for the farmer.

### Origin
A geographical area where coffee is from, typically a country.
- Name*: The name of the origin (e.g., "Ethiopia", "Colombia").
- Regions++: Specific regions within this origin.
- Description: Information about the coffee from this origin.

### Region (Place)
A specific coffee-growing region within an origin.
- Name*: The name of the region (e.g., "Yirgacheffe", "Antioquia").
- Origin+: The parent origin.
- Description: Information about the coffee from this region.

### Altitude
The elevation at which a coffee is grown.
- Elevation (meters)*: The elevation in meters above sea level. Can be a range.
- Description: How altitude affects the coffee.

### Variety
A specific cultivar of coffee plant.
- Name*: The name of the variety (e.g., "Gesha", "Bourbon", "Typica").
- Species: The species (e.g., "Arabica", "Robusta").
- Description: Information about the variety.

