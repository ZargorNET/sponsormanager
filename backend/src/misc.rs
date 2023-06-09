use anyhow::anyhow;

use crate::models::mongo::Sponsor;

macro_rules! return_err {
    ($ifex:expr, $($err:tt)+) => {{
        if $ifex {
            return Err(anyhow!(format!($($err)*)));
        }
    }};
}

pub fn assert_sponsor(sponsor: &Sponsor) -> anyhow::Result<()> {
    return_err!(sponsor.name.is_empty(), "name empty");
    return_err!(sponsor.name == "New Sponsor", "New Sponsor name is invalid");
    return_err!(sponsor.short_description.is_empty(), "description empty");
    for favour in sponsor.favours.iter() {
        return_err!(favour.condition.is_empty(), "favour empty");
    }
    for field in sponsor.fields.iter() {
        return_err!(field.name.is_empty(), "field name empty");
        return_err!(field.value.is_empty(), "field {} value empty", &field.name);
    }

    Ok(())
}
