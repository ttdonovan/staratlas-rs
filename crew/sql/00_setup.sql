CREATE SEQUENCE IF NOT EXISTS factions_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE IF NOT EXISTS factions (
    id INTEGER PRIMARY KEY DEFAULT nextval('factions_id'),
    name VARCHAR(128) UNIQUE NOT NULL
);

INSERT INTO factions (name) SELECT DISTINCT faction FROM 'tmp/private/crew.csv' ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS rarities_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE rarities (
    id INTEGER PRIMARY KEY DEFAULT nextval('rarities_id'),
    name VARCHAR(128) UNIQUE NOT NULL
);

INSERT INTO rarities (name) SELECT DISTINCT rarity FROM 'tmp/private/crew.csv' ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS species_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE species (
    id INTEGER PRIMARY KEY DEFAULT nextval('species_id'),
    name VARCHAR(128) UNIQUE NOT NULL
);

INSERT INTO species (name) SELECT DISTINCT species FROM 'tmp/private/crew.csv' ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS universities_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE universities (
    id INTEGER PRIMARY KEY DEFAULT nextval('universities_id'),
    name VARCHAR(128) UNIQUE NOT NULL
);

INSERT INTO universities (name) SELECT DISTINCT university FROM 'tmp/private/crew.csv' ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS crew_members_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE crew_members (
    id INTEGER PRIMARY KEY DEFAULT nextval('crew_members_id'),
    uid VARCHAR(128) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL,
    age DECIMAL(5, 2) NOT NULL,
    sex VARCHAR(10) NOT NULL,
    agreeableness DECIMAL(3, 2) NOT NULL,
    conscientiousness DECIMAL(3, 2) NOT NULL,
    extraversion DECIMAL(3, 2) NOT NULL,
    neuroticism DECIMAL(3, 2) NOT NULL,
    openness DECIMAL(3, 2) NOT NULL,
    faction_id INTEGER NOT NULL,
    rarity_id INTEGER NOT NULL,
    species_id INTEGER NOT NULL,
    university_id INTEGER NOT NULL,
    FOREIGN KEY (faction_id) REFERENCES factions(id),
    FOREIGN KEY (rarity_id) REFERENCES rarities(id),
    FOREIGN KEY (species_id) REFERENCES species(id),
    FOREIGN KEY (university_id) REFERENCES universities(id)
);

-- Step 1: Create a temporary table
CREATE TEMP TABLE tmp_crew (
    uid VARCHAR(128),
    name VARCHAR(128),
    age DECIMAL(5, 2),
    sex VARCHAR(10),
    agreeableness DECIMAL(3, 2),
    conscientiousness DECIMAL(3, 2),
    extraversion DECIMAL(3, 2),
    neuroticism DECIMAL(3, 2),
    openness DECIMAL(3, 2),
    faction_name VARCHAR(128),
    rarity_name VARCHAR(128),
    species_name VARCHAR(128),
    university_name VARCHAR(128)
);

-- Step 2: Load the CSV data into the temporary table
INSERT INTO tmp_crew (
    uid, name, age, sex, agreeableness, conscientiousness, extraversion, neuroticism, openness, faction_name, rarity_name, species_name, university_name
) SELECT id, name, age, sex, agreeableness, conscientiousness, extraversion, neuroticism, openness, faction, rarity, species, university FROM 'tmp/private/crew.csv';

-- Step 3: Insert data into the crew_members table with foreign key lookups
INSERT INTO crew_members (
    uid, name, age, sex, agreeableness, conscientiousness, extraversion, neuroticism, openness, faction_id, rarity_id, species_id, university_id
)
SELECT
    tmp.uid,
    tmp.name,
    tmp.age,
    tmp.sex,
    tmp.agreeableness,
    tmp.conscientiousness,
    tmp.extraversion,
    tmp.neuroticism,
    tmp.openness,
    (SELECT id FROM factions WHERE name = tmp.faction_name),
    (SELECT id FROM rarities WHERE name = tmp.rarity_name),
    (SELECT id FROM species WHERE name = tmp.species_name),
    (SELECT id FROM universities WHERE name = tmp.university_name)
FROM tmp_crew tmp
ON CONFLICT DO NOTHING;

DROP TABLE tmp_crew;

CREATE SEQUENCE IF NOT EXISTS aptitude_perks_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE aptitude_perks (
    id INTEGER PRIMARY KEY DEFAULT nextval('aptitude_perks_id'),
    name VARCHAR(128) UNIQUE NOT NULL
);

INSERT INTO aptitude_perks (name)
SELECT DISTINCT perk
FROM (
    SELECT aptitude_perk_1 AS perk FROM 'tmp/private/crew.csv' WHERE aptitude_perk_1 IS NOT NULL
    UNION ALL
    SELECT aptitude_perk_2 AS perk FROM 'tmp/private/crew.csv' WHERE aptitude_perk_2 IS NOT NULL
    UNION ALL
    SELECT aptitude_perk_3 AS perk FROM 'tmp/private/crew.csv' WHERE aptitude_perk_3 IS NOT NULL
) AS combined_perks
ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS aptitude_gains_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE aptitude_gains (
    id INTEGER PRIMARY KEY DEFAULT nextval('aptitude_gains_id'),
    name VARCHAR(128) NOT NULL
);

INSERT INTO aptitude_gains (name)
SELECT DISTINCT gain
FROM (
    SELECT aptitude_gain_1 AS gain FROM 'tmp/private/crew.csv' WHERE aptitude_gain_1 IS NOT NULL
    UNION ALL
    SELECT aptitude_gain_2 AS gain FROM 'tmp/private/crew.csv' WHERE aptitude_gain_2 IS NOT NULL
    UNION ALL
    SELECT aptitude_gain_3 AS gain FROM 'tmp/private/crew.csv' WHERE aptitude_gain_3 IS NOT NULL
) AS combined_gains
ON CONFLICT DO NOTHING;

CREATE SEQUENCE IF NOT EXISTS aptitudes_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE aptitudes (
    id INTEGER PRIMARY KEY DEFAULT nextval('aptitudes_id'),
    crew_member_id INTEGER,
    aptitude_perk_id INTEGER,
    aptitude_gain_id INTEGER,
    FOREIGN KEY (crew_member_id) REFERENCES crew_members(id),
    FOREIGN KEY (aptitude_perk_id) REFERENCES aptitude_perks(id),
    FOREIGN KEY (aptitude_gain_id) REFERENCES aptitude_gains(id)
);

-- Step 1: Create a temporary table
CREATE TEMP TABLE tmp_aptitudes (
    uid VARCHAR(128),
    perk_1_name VARCHAR(128),
    gain_1_name VARCHAR(128),
    perk_2_name VARCHAR(128),
    gain_2_name VARCHAR(128),
    perk_3_name VARCHAR(128),
    gain_3_name VARCHAR(128)
);

-- Step 2: Load the CSV data into the temporary table
INSERT INTO tmp_aptitudes (
    uid, perk_1_name, gain_1_name, perk_2_name, gain_2_name, perk_3_name, gain_3_name
) SELECT id, aptitude_perk_1, aptitude_gain_1, aptitude_perk_2, aptitude_gain_2, aptitude_perk_3, aptitude_gain_3 FROM 'tmp/private/crew.csv';

-- Step 3: Insert data into the aptitudes table with foreign key lookups
INSERT INTO aptitudes (
    crew_member_id, aptitude_perk_id, aptitude_gain_id
)
SELECT
    (SELECT id FROM crew_members WHERE uid = tmp.uid),
    (SELECT id FROM aptitude_perks WHERE name = tmp.perk_1_name),
    (SELECT id FROM aptitude_gains WHERE name = tmp.gain_1_name),
FROM tmp_aptitudes tmp
ON CONFLICT DO NOTHING;

INSERT INTO aptitudes (
    crew_member_id, aptitude_perk_id, aptitude_gain_id
)
SELECT
    (SELECT id FROM crew_members WHERE uid = tmp.uid),
    (SELECT id FROM aptitude_perks WHERE name = tmp.perk_2_name),
    (SELECT id FROM aptitude_gains WHERE name = tmp.gain_2_name),
FROM tmp_aptitudes tmp
ON CONFLICT DO NOTHING;

INSERT INTO aptitudes (
    crew_member_id, aptitude_perk_id, aptitude_gain_id
)
SELECT
    (SELECT id FROM crew_members WHERE uid = tmp.uid),
    (SELECT id FROM aptitude_perks WHERE name = tmp.perk_3_name),
    (SELECT id FROM aptitude_gains WHERE name = tmp.gain_3_name),
FROM tmp_aptitudes tmp
ON CONFLICT DO NOTHING;

DROP TABLE tmp_aptitudes;

CREATE SEQUENCE IF NOT EXISTS skills_id
    INCREMENT BY 1 MINVALUE 1;

CREATE TABLE skills (
    id INTEGER PRIMARY KEY DEFAULT nextval('skills_id'),
    crew_member_id INTEGER UNIQUE,
    command BOOLEAN,
    flight BOOLEAN,
    engineering BOOLEAN,
    medical BOOLEAN,
    science BOOLEAN,
    operator BOOLEAN,
    hospitality BOOLEAN,
    fitness BOOLEAN,
    FOREIGN KEY (crew_member_id) REFERENCES crew_members(id)
);

-- Step 1: Create a temporary table
CREATE TEMP TABLE tmp_skills (
    uid VARCHAR(128),
    command BOOLEAN,
    flight BOOLEAN,
    engineering BOOLEAN,
    medical BOOLEAN,
    science BOOLEAN,
    operator BOOLEAN,
    hospitality BOOLEAN,
    fitness BOOLEAN,
); 

-- Step 2: Load the CSV data into the temporary table
INSERT INTO tmp_skills (
    uid, command, flight, engineering, medical, science, operator, hospitality, fitness
) SELECT id, command, flight, engineering, medical, science, operator, hospitality, fitness FROM 'tmp/private/crew.csv';

-- Step 3: Insert data into the skills table with foreign key lookups
INSERT INTO skills (
    crew_member_id, command, flight, engineering, medical, science, operator, hospitality, fitness
)
SELECT
    (SELECT id FROM crew_members WHERE uid = tmp.uid),
    tmp.command,
    tmp.flight,
    tmp.engineering,
    tmp.medical,
    tmp.science,
    tmp.operator,
    tmp.hospitality,
    tmp.fitness
FROM tmp_skills tmp
ON CONFLICT DO NOTHING;

DROP TABLE tmp_skills;