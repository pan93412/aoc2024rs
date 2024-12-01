use polars::prelude::*;

fn main() {
    let df = df!(
        "column_1" => [1, 2, 3, 4, 5],
        "column_4" => [5, 4, 3, 2, 1]
    ).unwrap();

    dbg!(df.column("column_1").unwrap().as_series().unwrap().value_counts(false, false, "counts".into(), false).unwrap());
}
