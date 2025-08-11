use serde::Serialize;

#[derive(Serialize)]
pub struct ReturnedError {
    error: String,
}

impl<T> From<T> for ReturnedError
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self {
            error: value.to_string(),
        }
    }
}

// turn chrono NaiveDateTime (as sea_orm::prelude::DateTime) into ISO 8601 and back
pub mod naive_utc_rfc3339 {
    use sea_orm::prelude::{DateTime, DateTimeWithTimeZone};
    use sea_orm::sqlx::types::chrono::FixedOffset;
    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let utc_dt: DateTimeWithTimeZone =
            DateTimeWithTimeZone::from_naive_utc_and_offset(*dt, FixedOffset::east_opt(0).unwrap());
        serializer.serialize_str(&utc_dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let utc_dt = DateTimeWithTimeZone::parse_from_rfc3339(&s).map_err(de::Error::custom)?;
        Ok(utc_dt.naive_utc())
    }
}

// why do i have to do this :(
pub mod naive_utc_rfc3339_opt {
    use crate::util::naive_utc_rfc3339;
    use sea_orm::prelude::{DateTime, DateTimeWithTimeZone};
    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => naive_utc_rfc3339::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        opt.map(|s| {
            let utc_dt = DateTimeWithTimeZone::parse_from_rfc3339(&s).map_err(de::Error::custom)?;
            Ok(utc_dt.naive_utc())
        })
        .transpose()
    }
}

pub mod active_enum {
    use sea_orm::ActiveEnum;
    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<A, S>(active_enum: &A, serializer: S) -> Result<S::Ok, S::Error>
    where
        A: ActiveEnum<Value = String>,
        S: Serializer,
    {
        let s = active_enum.to_value();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, A, D>(deserializer: D) -> Result<A, D::Error>
    where
        A: ActiveEnum<Value = String>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let Some(variant) = A::values().into_iter().find(|v| v == &s) else {
            return Err(de::Error::custom("not a variant"));
        };

        Ok(A::try_from_value(&variant).unwrap())
    }
}
