use chrono::NaiveDate;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df: DataFrame = df!(
        "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
        "birthdate" => [
            NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
            NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
            NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
        ],
        "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
        "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
    )
    .unwrap();

    let expr = col("weight") / (col("height").pow(2));

    let df1 = df
        .clone()
        .lazy()
        .with_columns([
            ((expr.clone() - expr.clone().mean()) / expr.clone().std(1)).alias("deviation")
        ])
        .collect()?;
    println!("{:?}", df1);

    let df2 = df
        .clone()
        .lazy()
        .with_columns([
            expr.clone().alias("bmi"),
            expr.clone().mean().alias("mean_bmi"),
        ])
        .collect()?;

    println!("{:?}", df2);
    Ok(())
}
