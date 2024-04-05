use std::cmp::min;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal_macros::dec;
use sea_orm::prelude::Decimal;

use crate::Error;

pub fn calculate_potential(score: i32, chart_constant: Decimal) -> Result<Decimal, Error> {
    if chart_constant < dec!(0) {
        return Err("chart constant cannot be negative.".into());
    }

    let score = min(score, 10_000_000);

    if score >= 10_000_000 {
        return Ok(chart_constant + dec!(2.0));
    }

    if score >= 9_800_000 {
        let value = 1.0 + (score - 9_800_000) as f32 / 200_000.0;
        return Ok(chart_constant + Decimal::from_f32(value).unwrap());
    }

    let potential =
        chart_constant + Decimal::from_f32((score - 9_500_000) as f32 / 300_000.0).unwrap();

    if potential > dec!(0) {
        Ok(potential)
    } else {
        Ok(dec!(0))
    }
}
