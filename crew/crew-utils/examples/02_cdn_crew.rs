use duckdb::{params, Connection};
use serde::Deserialize;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use std::collections::HashMap;
use std::env;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
enum Faction {
    MUD,
    ONI,
    Ustur,
    Unaligned,
}

impl Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::MUD => write!(f, "MUD"),
            Faction::ONI => write!(f, "ONI"),
            Faction::Ustur => write!(f, "Ustur"),
            Faction::Unaligned => write!(f, "Unaligned"),
        }
    }
}

#[derive(Debug, Deserialize)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Anomaly,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rarity::Common => write!(f, "Common"),
            Rarity::Uncommon => write!(f, "Uncommon"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::Epic => write!(f, "Epic"),
            Rarity::Legendary => write!(f, "Legendary"),
            Rarity::Anomaly => write!(f, "Anomaly"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Aptitudes(HashMap<AptitudePerks, AptitudeGains>);

#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
enum AptitudePerks {
    Command,
    Flight,
    Engineering,
    Medical,
    Science,
    Operator,
    Hospitality,
    Fitness,
}

impl Display for AptitudePerks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AptitudePerks::Command => write!(f, "Command"),
            AptitudePerks::Flight => write!(f, "Flight"),
            AptitudePerks::Engineering => write!(f, "Engineering"),
            AptitudePerks::Medical => write!(f, "Medical"),
            AptitudePerks::Science => write!(f, "Science"),
            AptitudePerks::Operator => write!(f, "Operator"),
            AptitudePerks::Hospitality => write!(f, "Hospitality"),
            AptitudePerks::Fitness => write!(f, "Fitness"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum AptitudeGains {
    Minor,
    Major,
    Anomaly, // can this be removed?
    Anomalous,
}

impl Display for AptitudeGains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AptitudeGains::Minor => write!(f, "Minor"),
            AptitudeGains::Major => write!(f, "Major"),
            AptitudeGains::Anomaly => write!(f, "Anomaly"),
            AptitudeGains::Anomalous => write!(f, "Anomalous"),
        }
    }
}

#[derive(Debug)]
struct Skills {
    command: bool,
    flight: bool,
    engineering: bool,
    medical: bool,
    science: bool,
    operator: bool,
    hospitality: bool,
    fitness: bool,
}

impl From<&Aptitudes> for Skills {
    fn from(aptitudes: &Aptitudes) -> Self {
        Skills {
            command: aptitudes.0.contains_key(&AptitudePerks::Command),
            flight: aptitudes.0.contains_key(&AptitudePerks::Flight),
            engineering: aptitudes.0.contains_key(&AptitudePerks::Engineering),
            medical: aptitudes.0.contains_key(&AptitudePerks::Medical),
            science: aptitudes.0.contains_key(&AptitudePerks::Science),
            operator: aptitudes.0.contains_key(&AptitudePerks::Operator),
            hospitality: aptitudes.0.contains_key(&AptitudePerks::Hospitality),
            fitness: aptitudes.0.contains_key(&AptitudePerks::Fitness),
        }
    }
}

#[derive(Debug)]
struct Crew {
    mint_offset: u32,
    rarity: Rarity,
    faction: Faction,
    species: String,
    sex: String,
    name: String,
    university: String,
    age: f64,
    openness: f64,
    conscientiousness: f64,
    extraversion: f64,
    agreeableness: f64,
    neuroticism: f64,
    aptitudes: Aptitudes,
    skills: Skills,
    image_url: String,
    das_id: Option<String>,
}

impl Crew {
    fn aptitude_tags(&self) -> String {
        self.aptitudes
            .0
            .iter()
            .map(|(k, v)| format!("<{}-{}>", k, v).to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("")
    }

    fn aptitude_perks(&self) -> String {
        self.aptitudes
            .0
            .keys()
            .map(|k| format!("<{}>", k).to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("")
    }

    fn aptitude_gains(&self) -> String {
        self.aptitudes
            .0
            .values()
            .map(|v| format!("<{}>", v).to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<'de> Deserialize<'de> for Crew {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CrewHelper {
            mint_offset: u32,
            rarity: Rarity,
            faction: Faction,
            species: String,
            sex: String,
            name: String,
            university: String,
            age: f64,
            agreeableness: f64,
            conscientiousness: f64,
            extraversion: f64,
            neuroticism: f64,
            openness: f64,
            aptitudes: Aptitudes,
            image_url: String,
            #[serde(rename = "dasID")]
            das_id: Option<String>, // what does this field mean?
        }

        let helper = CrewHelper::deserialize(deserializer)?;
        let skills = Skills::from(&helper.aptitudes);

        Ok(Crew {
            mint_offset: helper.mint_offset,
            faction: helper.faction,
            species: helper.species,
            sex: helper.sex,
            name: helper.name,
            university: helper.university,
            age: helper.age,
            openness: helper.openness,
            conscientiousness: helper.conscientiousness,
            extraversion: helper.extraversion,
            agreeableness: helper.agreeableness,
            neuroticism: helper.neuroticism,
            rarity: helper.rarity,
            aptitudes: helper.aptitudes,
            skills,
            image_url: helper.image_url,
            das_id: helper.das_id,
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // curl -o tmp/crew.json https://cdn.staratlas.com/crew.json
    // wc -l tmp/crew.json
    // 693512

    // du -h tmp/crew.json
    // 396M    tmp/crew.json

    // cargo build --release -p crew-utils --example 02_cdn_crew
    // time cargo run --release -p crew-utils --example 02_cdn_crew tmp/crew.json tmp/crew.db
    // real    51m43.936s

    // du -h tmp/crew.db.parquet/crew.parquet
    // 28M     tmp/crew.db.parquet/crew.parquet

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> <db_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let db_path = &args[2];

    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        r#"
        DROP TABLE IF EXISTS crew;

        CREATE TABLE crew (
            mint_offset INTEGER,
            rarityName VARCHAR,
            factionName VARCHAR,
            speciesName VARCHAR,
            sexName VARCHAR,
            aptitudeTags VARCHAR,
            aptitudePerks VARCHAR,
            aptitudeGains VARCHAR,
            name VARCHAR,
            age DOUBLE,
            agreeableness DOUBLE,
            conscientiousness DOUBLE,
            extraversion DOUBLE,
            neuroticism DOUBLE,
            openness DOUBLE,
            is_command BOOLEAN,
            is_flight BOOLEAN,
            is_engineering BOOLEAN,
            is_medical BOOLEAN,
            is_science BOOLEAN,
            is_operator BOOLEAN,
            is_hospitality BOOLEAN,
            is_fitness BOOLEAN
        );
    "#,
    )?;

    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        // dbg!(&line);
        let crew: Crew = serde_json::from_str(&line)?;
        // dbg!(&crew);

        conn.execute(
            "INSERT INTO crew (
                mint_offset,
                rarityName,
                factionName,
                speciesName,
                sexName,
                aptitudeTags,
                aptitudePerks,
                aptitudeGains,
                name,
                age,
                agreeableness,
                conscientiousness,
                extraversion,
                neuroticism,
                openness,
                is_command,
                is_flight,
                is_engineering,
                is_medical,
                is_science,
                is_operator,
                is_hospitality,
                is_fitness
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                crew.mint_offset,
                crew.rarity.to_string(),
                crew.faction.to_string(),
                crew.species,
                crew.sex,
                crew.aptitude_tags(),
                crew.aptitude_perks(),
                crew.aptitude_gains(),
                crew.name,
                crew.age,
                crew.agreeableness,
                crew.conscientiousness,
                crew.extraversion,
                crew.neuroticism,
                crew.openness,
                crew.skills.command,
                crew.skills.flight,
                crew.skills.engineering,
                crew.skills.medical,
                crew.skills.science,
                crew.skills.operator,
                crew.skills.hospitality,
                crew.skills.fitness,
            ],
        )?;
    }

    let batch = format!(
        r#"
        DROP TYPE IF EXISTS rarity;
        CREATE TYPE rarity AS ENUM (SELECT DISTINCT lower(rarityName) AS rarity FROM crew ORDER BY rarity ASC);

        DROP TYPE IF EXISTS faction;
        CREATE TYPE faction AS ENUM (SELECT DISTINCT lower(factionName) AS faction FROM crew ORDER BY faction ASC);

        DROP TYPE IF EXISTS species;
        CREATE TYPE species AS ENUM (SELECT DISTINCT lower(replace(speciesName, ' ', '-')) AS species FROM crew ORDER BY species ASC);

        DROP TYPE IF EXISTS sex;
        CREATE TYPE sex AS ENUM (SELECT DISTINCT lower(replace(sexName, ' ', '-')) AS sex FROM crew ORDER BY sex ASC);

        DROP TYPE IF EXISTS aptitude;
        CREATE TYPE aptitude AS ENUM (SELECT DISTINCT unnest(split(aptitudeTags[2:-2], '><')) AS tag FROM crew ORDER BY tag ASC);

        DROP TYPE IF EXISTS perk;
        CREATE TYPE perk AS ENUM (SELECT DISTINCT unnest(split(aptitudePerks[2:-2], '><')) AS perk FROM crew ORDER BY perk ASC);

        DROP TYPE IF EXISTS gain;
        CREATE TYPE gain AS ENUM (SELECT DISTINCT unnest(split(aptitudeGains[2:-2], '><')) AS gain FROM crew ORDER BY gain ASC);

        ALTER TABLE crew ADD COLUMN rarity rarity;
        ALTER TABLE crew ADD COLUMN faction faction;
        ALTER TABLE crew ADD COLUMN species species;
        ALTER TABLE crew ADD COLUMN sex sex;
        ALTER TABLE crew ADD COLUMN aptitudes aptitude[];
        ALTER TABLE crew ADD COLUMN aptitude_perks perk[];
        ALTER TABLE crew ADD COLUMN aptitude_gains gain[];

        CREATE OR REPLACE TABLE aptitude_tags AS
        SELECT
            mint_offset,
            array_agg(tag) AS tags
        FROM (
            SELECT
                mint_offset,
                unnest(split(aptitudeTags[2:-2], '><')) AS tag 
            FROM crew
        ) AS flattened_tags
        GROUP BY mint_offset;

        CREATE OR REPLACE TABLE aptitude_perks AS
        SELECT
            mint_offset,
            array_agg(perk) AS perks
        FROM (
            SELECT
                mint_offset,
                unnest(split(aptitudePerks[2:-2], '><')) AS perk
            FROM crew
        ) AS flattened_perks
        GROUP BY mint_offset;

        CREATE OR REPLACE TABLE aptitude_gains AS
        SELECT
            mint_offset,
            array_agg(DISTINCT gain) AS gains
        FROM (
            SELECT
                mint_offset,
                unnest(split(aptitudeGains[2:-2], '><')) AS gain
            FROM crew
        ) AS flattened_gains
        GROUP BY mint_offset;

        UPDATE crew SET
            rarity = lower(rarityName),
            faction = lower(factionName),
            species = lower(replace(speciesName, ' ', '-')),
            sex = lower(replace(sexName, ' ', '-'));

        UPDATE crew SET
            aptitudes = (SELECT tags FROM aptitude_tags WHERE aptitude_tags.mint_offset = crew.mint_offset),
            aptitude_perks = (SELECT perks FROM aptitude_perks WHERE aptitude_perks.mint_offset = crew.mint_offset),
            aptitude_gains = (SELECT gains FROM aptitude_gains WHERE aptitude_gains.mint_offset = crew.mint_offset);
        
        DROP TABLE IF EXISTS aptitude_tags;
        DROP TABLE IF EXISTS aptitude_perks;
        DROP TABLE IF EXISTS aptitude_gains;

        ALTER TABLE crew DROP COLUMN rarityName;
        ALTER TABLE crew DROP COLUMN factionName;
        ALTER TABLE crew DROP COLUMN speciesName;
        ALTER TABLE crew DROP COLUMN sexName;
        ALTER TABLE crew DROP COLUMN aptitudeTags;
        ALTER TABLE crew DROP COLUMN aptitudePerks;
        ALTER TABLE crew DROP COLUMN aptitudeGains;

        EXPORT DATABASE '{db_path}.parquet' (FORMAT 'parquet', CODEC 'SNAPPY');
    "#
    );

    conn.execute_batch(&batch)?;

    Ok(())
}
