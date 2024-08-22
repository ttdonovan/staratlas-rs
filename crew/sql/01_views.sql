CREATE VIEW IF NOT EXISTS v_command_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.command = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Command';

CREATE VIEW IF NOT EXISTS v_flight_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.flight = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Flight';

CREATE VIEW IF NOT EXISTS v_engineering_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.engineering = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Engineering';

CREATE VIEW IF NOT EXISTS v_medical_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Medical';

CREATE VIEW IF NOT EXISTS v_medical_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.medical = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Medical';

CREATE VIEW IF NOT EXISTS v_science_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.science = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Science';

CREATE VIEW IF NOT EXISTS v_operator_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.operator = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Operator';

CREATE VIEW IF NOT EXISTS v_hospitality_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.hospitality = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Hospitality';

CREATE VIEW IF NOT EXISTS v_fitness_crew AS
SELECT
    c.id,
    c.uid,
    c.name,
    p.name AS perk,
    g.name AS gain,
    c.agreeableness,
    (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true) AS mean_agreeableness,
    ROUND(c.agreeableness - (SELECT ROUND(MEAN(c.agreeableness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true), 2) AS diff_agreeableness,
    c.conscientiousness,
    (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true) AS mean_conscientiousness,
    ROUND(c.conscientiousness - (SELECT ROUND(MEAN(c.conscientiousness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true), 2) AS diff_conscientiousness,
    c.extraversion,
    (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true) AS mean_extraversion,
    ROUND(c.extraversion - (SELECT ROUND(MEAN(c.extraversion), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true), 2) AS diff_extraversion,
    c.neuroticism,
    (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true) AS mean_neuroticism,
    ROUND(c.neuroticism - (SELECT ROUND(MEAN(c.neuroticism), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true), 2) AS diff_neuroticism,
    c.openness,
    (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true) AS mean_openness,
    ROUND(c.openness - (SELECT ROUND(MEAN(c.openness), 2)
     FROM crew_members AS c
     JOIN skills AS s ON s.crew_member_id = c.id
     WHERE s.fitness = true), 2) AS diff_openness
FROM crew_members AS c
JOIN skills AS s ON s.crew_member_id = c.id
JOIN aptitudes AS a ON a.crew_member_id = c.id
JOIN aptitude_perks AS p ON p.id = a.aptitude_perk_id
JOIN aptitude_gains AS g ON g.id = a.aptitude_gain_id
WHERE p.name = 'Fitness';