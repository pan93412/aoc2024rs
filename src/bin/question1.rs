use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = LazyCsvReader::new("./inputs/01.txt")
        .with_has_header(false)
        .with_separator(b' ')
        .finish()?;

    // Data cleaning
    let df = df.select([
        col("column_1").alias("left").sort(SortOptions::new()),
        col("column_4").alias("right").sort(SortOptions::new()),
    ]);

    // Question 1
    let q1 = df.clone().with_column(
        (col("left") - col("right")).abs().alias("distance")
    );

    let distance_sum = q1.select([col("distance")]).sum();

    println!("Question 1 answer: {:?}", distance_sum.collect()?.column("distance")?.i64()?.first().expect("must has a value"));

    // Question 2
    let q2 = df.collect()?;
    let left = q2.column("left")?.as_series().expect("must be a series");
    let right = q2.column("right")?.as_series().expect("must be a series");

    println!("{:?}", left.value_counts(false, false, "counts".into(), false)?);


    // // Data cleaning
    // let left = df.column("column_1")?;
    // let right = df.column("column_4")?;

    // let sorted_left = left.sort(SortOptions::default())?.with_name("left".into());
    // let sorted_right = right.sort(SortOptions::default())?.with_name("right".into());

    // let sorted_df = DataFrame::new(vec![sorted_left, sorted_right])?;

    // // Question 1
    // let v = polars::abs((sorted_df.column("left")? - sorted_df.column("right")?)?);

    // let df_with_sum = sorted_df.with_column(
    //     Column::new("sum".into(), sorted_df.column("left")? + sorted_df.column("right")?)
    // );


    // println!("{:?}", df);

    Ok(())
}
