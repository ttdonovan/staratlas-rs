use polars::prelude::*;

use std::env;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprintln!("Usage: {} <parquet_path>", args[0]);
        std::process::exit(1);
    }
    let parquet_path = &args[1];

    let mut file = File::open(parquet_path)?;
    let df = ParquetReader::new(&mut file).finish()?;

    // let out = df
    //     .clone()
    //     .lazy()
    //     .select([
    //         col("mint_offset"),
    //         col("rarity"),
    //         col("name"),
    //         col("aptitudes"),
    //     ])
    //     .collect()?;

    // println!("{}", out);

    let overall_mean = df
        .clone()
        .lazy()
        .select([
            mean("agreeableness").alias("overall_agreeableness"),
            mean("conscientiousness").alias("overall_conscientiousness"),
            mean("extraversion").alias("overall_extraversion"),
            mean("neuroticism").alias("overall_neuroticism"),
            mean("openness").alias("overall_openness"),
        ])
        .collect()?;

    println!("{}", &overall_mean);

    let grouped_df = df
        .clone()
        .lazy()
        .group_by(["rarity"])
        .agg([
            col("rarity").count().alias("freq"),
            mean("agreeableness").alias("mean_agreeableness"),
            mean("conscientiousness").alias("mean_conscientiousness"),
            mean("extraversion").alias("mean_extraversion"),
            mean("neuroticism").alias("mean_neuroticism"),
            mean("openness").alias("mean_openness"),
        ])
        .collect()?;

    println!("{}", &grouped_df);

    let joined_df = grouped_df
        .clone()
        .lazy()
        .cross_join(overall_mean.clone().lazy(), None)
        .collect()?;

    println!("{}", &joined_df);

    let comparison_df = joined_df
        .clone()
        .lazy()
        .with_column(
            (col("mean_agreeableness") - col("overall_agreeableness")).alias("diff_agreeableness"),
        )
        .with_column(
            (col("mean_conscientiousness") - col("overall_conscientiousness"))
                .alias("diff_conscientiousness"),
        )
        .with_column(
            (col("mean_extraversion") - col("overall_extraversion")).alias("diff_extraversion"),
        )
        .with_column(
            (col("mean_neuroticism") - col("overall_neuroticism")).alias("diff_neuroticism"),
        )
        .with_column((col("mean_openness") - col("overall_openness")).alias("diff_openness"))
        .collect()?;

    println!(
        "{}",
        comparison_df
            .clone()
            .lazy()
            .select([
                col("rarity"),
                col("freq"),
                col("diff_agreeableness"),
                col("diff_conscientiousness"),
                col("diff_extraversion"),
                col("diff_neuroticism"),
                col("diff_openness"),
            ])
            .collect()?
    );

    let attributes_df = df
        .clone()
        .lazy()
        .group_by(["rarity"])
        .agg([
            len().alias("freq"),
            col("is_command").sum().alias("command_count"),
            col("is_flight").sum().alias("flight_count"),
            col("is_engineering").sum().alias("engineering_count"),
            col("is_medical").sum().alias("medical_count"),
            col("is_science").sum().alias("science_count"),
            col("is_operator").sum().alias("operator_count"),
            col("is_hospitality").sum().alias("hospitality_count"),
            col("is_fitness").sum().alias("fitness_count"),
        ])
        .collect()?;

    println!("{}", &attributes_df);

    Ok(())
}
