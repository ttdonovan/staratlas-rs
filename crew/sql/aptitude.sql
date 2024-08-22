SELECT COUNT(*) as freq, perk, gain, bar(freq, 0, 100, 20) as plot
FROM (
    SELECT aptitude_perk_1 AS perk, aptitude_gain_1 AS gain FROM 'tmp/crew.csv'
    UNION ALL
    SELECT aptitude_perk_2 AS perk, aptitude_gain_2 AS gain  FROM 'tmp/crew.csv'
    UNION ALL
    SELECT aptitude_perk_3 AS perk, aptitude_gain_3 AS gain  FROM 'tmp/crew.csv'
) AS combined_perks_and_gains
GROUP BY perk, gain
ORDER BY perk, gain DESC;

select
    count(id) as freq,
    rarity,
    round(sum(case when command then 1 else 0 end) * 100.0 / count(id), 2) AS command_percentage,
    round(sum(case when flight then 1 else 0 end) * 100.0 / count(id), 2) AS flight_percentage,
    round(sum(case when engineering then 1 else 0 end) * 100.0 / count(id), 2) AS engineering_percentage,
    round(sum(case when medical then 1 else 0 end) * 100.0 / count(id), 2) AS medical_percentage,
    round(sum(case when science then 1 else 0 end) * 100.0 / count(id), 2) AS science_percentage,
    round(sum(case when operator then 1 else 0 end) * 100.0 / count(id), 2) AS operator_percentage,
    round(sum(case when hospitality then 1 else 0 end) * 100.0 / count(id), 2) AS hospitality_percentage,
    round(sum(case when fitness then 1 else 0 end) * 100.0 / count(id), 2) AS fitness_percentage
from 'tmp/crew.csv'
group by rarity
order by freq desc;

WITH total_count AS (
    SELECT COUNT(id) AS total
    FROM 'tmp/crew.csv'
)
SELECT
    COUNT(id) AS freq,
    rarity,
    ROUND(SUM(CASE WHEN command THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS command_percentage,
    ROUND(SUM(CASE WHEN flight THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS flight_percentage,
    ROUND(SUM(CASE WHEN engineering THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS engineering_percentage,
    ROUND(SUM(CASE WHEN medical THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS medical_percentage,
    ROUND(SUM(CASE WHEN science THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS science_percentage,
    ROUND(SUM(CASE WHEN operator THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS operator_percentage,
    ROUND(SUM(CASE WHEN hospitality THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS hospitality_percentage,
    ROUND(SUM(CASE WHEN fitness THEN 1 ELSE 0 END) * 100.0 / COUNT(id), 2) AS fitness_percentage,
    ROUND(SUM(CASE WHEN command THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS command_percentage_total,
    ROUND(SUM(CASE WHEN flight THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS flight_percentage_total,
    ROUND(SUM(CASE WHEN engineering THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS engineering_percentage_total,
    ROUND(SUM(CASE WHEN medical THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS medical_percentage_total,
    ROUND(SUM(CASE WHEN science THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS science_percentage_total,
    ROUND(SUM(CASE WHEN operator THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS operator_percentage_total,
    ROUND(SUM(CASE WHEN hospitality THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS hospitality_percentage_total,
    ROUND(SUM(CASE WHEN fitness THEN 1 ELSE 0 END) * 100.0 / (SELECT total FROM total_count), 2) AS fitness_percentage_total
FROM 'tmp/crew.csv'
GROUP BY rarity
ORDER BY freq DESC;