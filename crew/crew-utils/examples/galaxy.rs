use anchor_lang::{prelude::Pubkey, pubkey};
use csv_async::AsyncSerializer;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use std::collections::HashMap;

const WALLET: Pubkey = pubkey!("2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77");

const GALAXY_API: &str = "https://galaxy.staratlas.com";

#[derive(Debug, serde::Deserialize)]
struct CrewResponse(Vec<Crew>);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum Faction {
    MUD,
    ONI,
    Ustur,
    Unaligned,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Anomaly,
}

#[derive(Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
enum Perk {
    Command,
    Flight,
    Engineering,
    Medical,
    Science,
    Operator,
    Hospitality,
    Fitness,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum GainRate {
    Minor,
    Major,
    Anomoly,
}

#[derive(Debug, serde::Deserialize)]
struct Crew {
    #[serde(rename = "_id")]
    id: String,
    faction: Faction,
    rarity: Rarity,
    name: String,
    age: f64,
    sex: String,
    species: String,
    university: String,
    agreeableness: f64,
    conscientiousness: f64,
    extraversion: f64,
    neuroticism: f64,
    openness: f64,
    aptitudes: HashMap<Perk, GainRate>,
}

#[derive(Debug, serde::Serialize)]
struct CsvRow {
    id: String,
    faction: Faction,
    rarity: Rarity,
    name: String,
    age: f64,
    sex: String,
    species: String,
    university: String,
    agreeableness: f64,
    conscientiousness: f64,
    extraversion: f64,
    neuroticism: f64,
    openness: f64,
    aptitude_perk_1: Option<Perk>,
    aptitude_gain_1: Option<GainRate>,
    aptitude_perk_2: Option<Perk>,
    aptitude_gain_2: Option<GainRate>,
    aptitude_perk_3: Option<Perk>,
    aptitude_gain_3: Option<GainRate>,
    command: bool,
    flight: bool,
    engineering: bool,
    medical: bool,
    science: bool,
    operator: bool,
    hospitality: bool,
    fitness: bool,
}

impl From<Crew> for CsvRow {
    fn from(crew: Crew) -> Self {
        let command = crew.aptitudes.contains_key(&Perk::Command);
        let flight = crew.aptitudes.contains_key(&Perk::Flight);
        let engineering = crew.aptitudes.contains_key(&Perk::Engineering);
        let medical = crew.aptitudes.contains_key(&Perk::Medical);
        let science = crew.aptitudes.contains_key(&Perk::Science);
        let operator = crew.aptitudes.contains_key(&Perk::Operator);
        let hospitality = crew.aptitudes.contains_key(&Perk::Hospitality);
        let fitness = crew.aptitudes.contains_key(&Perk::Fitness);

        let mut aptitudes = crew.aptitudes.into_iter();

        let (aptitude_perk_1, aptitude_gain_1) = match aptitudes.next() {
            Some((perk, gain)) => (Some(perk), Some(gain)),
            None => (None, None),
        };

        let (aptitude_perk_2, aptitude_gain_2) = match aptitudes.next() {
            Some((perk, gain)) => (Some(perk), Some(gain)),
            None => (None, None),
        };

        let (aptitude_perk_3, aptitude_gain_3) = match aptitudes.next() {
            Some((perk, gain)) => (Some(perk), Some(gain)),
            None => (None, None),
        };

        Self {
            id: crew.id,
            faction: crew.faction,
            rarity: crew.rarity,
            name: crew.name,
            age: crew.age,
            sex: crew.sex,
            species: crew.species,
            university: crew.university,
            agreeableness: crew.agreeableness,
            conscientiousness: crew.conscientiousness,
            extraversion: crew.extraversion,
            neuroticism: crew.neuroticism,
            openness: crew.openness,
            aptitude_perk_1,
            aptitude_perk_2,
            aptitude_perk_3,
            aptitude_gain_1,
            aptitude_gain_2,
            aptitude_gain_3,
            command,
            flight,
            engineering,
            medical,
            science,
            operator,
            hospitality,
            fitness,
        }
    }
}

/*
    Usage:

    $ cd crew
    $ pwd .
    /path/to/staratlas-rs/crew

    $ cat tmp/crew.txt
    4208
    4433
    5779

    $ cat ./tmp/crew.txt | cargo run -p crew-utils --example galaxy > ./tmp/crew.csv
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut wtr = AsyncSerializer::from_writer(vec![]);

    // read from stdin
    let reader = BufReader::new(io::stdin());
    let mut lines = reader.lines();

    let client = reqwest::Client::new();
    let uri = format!("{}/crew", GALAXY_API);

    while let Some(line) = lines.next_line().await? {
        let mint_offset = line.trim();
        let json: serde_json::Value = client
            .get(&uri)
            .query(&[("mintOffset", mint_offset)])
            .send()
            .await?
            .json()
            .await?;

        // dbg!(&json);

        let crew_resp: CrewResponse = serde_json::from_value(json)?;

        if let Some(crew) = crew_resp.0.into_iter().next() {
            // dbg!(&crew);
            let row = CsvRow::from(crew);
            // dbg!(row);

            wtr.serialize(row).await?;
        }
    }

    let data = String::from_utf8(wtr.into_inner().await?)?;
    print!("{}", data);

    Ok(())
}
