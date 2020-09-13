# Characters

A Character consists of:

- A Race
- A set of Ability scores
- A set of Skill values
- A set of Characteristics

## Abilities

Abilities represent the raw abilities of a character. These are things such as Strength and Intelligence.

Abilities all have a default value, and then the selected race or characteristics may adjust this either up or down.

## Skills

Skills represent the learned abilities of a character. These are things such as Longsword or Pickpocket.

Skills exist in a tree structure, where the leaves of the tree are the actual skills and the nodes are used for grouping. Every leaf can have a value, or can be left as Unassigned. Every node has a value that is the average of it's immediate children - where an Unassigned leaf has a value of 0.

For example:

- Fighting (2 = Avg(5, 0))
  - Melee (5 = Avg(10, 0))
    - Swords (10 = Avg(20, 0))
      - Longsword (20)
      - Shortsword (-)
    - Blunt (0 = Avg(0, 0))
      - Hammer (-)
      - Mace (-)
  - Ranged (0 = Avg(0, 0))
    - Longbow (-)
    - Crossbow (-)

When using an Unassigned skill, the value used is calculated as the first parent with a non-zero score divided by the number of steps needed to reach that parent. This means that:

- Longsword = 20
- Shortsword = 10 (10 / 1)
- Hammer = 2 (5 / 2)
- Longbow = 1 (2 / 2)

Thus the lack of any particular skill means that it can still be used, but significantly underpowered. The more ability in similar skills, the better this will be.

All Skills are associated with one or more Abilities. When performing skill checks, the average of these ability scores are used as well as the skill level. For example, Longsword might only depend on Strength, whereas Intimidation might depend on both Strength and Charisma.

### Side Note - Character Level

Character Level is _not_ a tracked concept. Instead it is the overall average of skills. Effectively the top level of the skill tree is the character level.

This is used for display purposes, but is also used for characteristics.

## Characteristics

Characteristics represent all other ways to improve a character. These can be similar to D20 feats, to underlying character abilities - e.g. Spellcaster - or to simply be increased skill or ability scores.

Every characteristic has a points cost, and a set of requirements. These requirements are some combination of:

- Ability score - max or min value
- Skill score - max or min value
- Presence or absence of other characteristics
- Particular race
- Whether or not we're in character creation

In addition, some characteristics are player selectable and some aren't.

Player Selectable Characteristics are purchased using Characteristic Points. These are calculated based on the Character Level, so as the character skills improve, more characteristic points are available to purchase characteristics with.

## Races

Races represent the race of the character. This has some affect in gameplay - certain things act differently for different races. This also affects the characteristics that are available to select.

Further, the race that is selected will cause different ability scores to be assigned during character creation.

## Character Creation

Character Creation consists of selecting:

- A Race
- Some Skill values
- Some Characteristics

### Character Classes

The presence of Character Classes can be used to automatically assign skill values and characteristics during creation. Alternatively, the player can choose to assign these manually.
