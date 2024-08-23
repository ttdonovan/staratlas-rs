summarize select agreeableness, conscientiousness, extraversion, neuroticism, openness from 'tmp/crew.csv';

select
    count(id) as freq,
    rarity,
    round(mean(agreeableness), 3) as mean_agreeableness,
    round(mean(conscientiousness), 3) as mean_conscientiousness,
    round(mean(extraversion), 3) as mean_extraversion,
    round(mean(neuroticism), 3) as mean_neuroticism,
    round(mean(openness), 3) as mean_openness
from 'tmp/private/crew.csv'
group by rarity
order by freq desc;

select
    rarity,
    count(mint_offset) as freq,
    round(mean(agreeableness), 3) as mean_agreeableness,
    round(mean(conscientiousness), 3) as mean_conscientiousness,
    round(mean(extraversion), 3) as mean_extraversion,
    round(mean(neuroticism), 3) as mean_neuroticism,
    round(mean(openness), 3) as mean_openness
from crew
where is_command = true
and list_contains(aptitudes, 'command-major')
group by 1;