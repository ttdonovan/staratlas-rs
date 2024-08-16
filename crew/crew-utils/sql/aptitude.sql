SELECT COUNT(*) as freq, perk, gain, bar(freq, 0, 10, 10)
FROM (
    SELECT aptitude_perk_1 AS perk, aptitude_gain_1 AS gain FROM 'tmp/private/crew.csv'
    UNION ALL
    SELECT aptitude_perk_2 AS perk, aptitude_gain_2 AS gain  FROM 'tmp/private/crew.csv'
    UNION ALL
    SELECT aptitude_perk_3 AS perk, aptitude_gain_3 AS gain  FROM 'tmp/private/crew.csv'
) AS combined_perks_and_gains
GROUP BY perk, gain
ORDER BY perk, gain DESC;