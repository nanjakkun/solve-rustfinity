/*
You need to implement the From trait for the following conversions:

    Minutes to Hours
    Hours to Days
    Minutes to Days
    Days to Hours
*/

pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

// TODO: implement from hours to days
impl From<Hours> for Days {
    fn from(hours: Hours) -> Days {
        Days(hours.0 / 24)
    }
}

// TODO: implement from minutes to days
impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        Hours(minutes.0 / 60)
    }
}

impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Days {
        Days(minutes.0 / 1440)
    }
}

// TODO: implement from days to hours
impl From<Days> for Hours {
    fn from(days: Days) -> Hours {
        Hours(days.0 * 24)
    }
}

// Example usage
pub fn main() {
    let minutes = Minutes(60);
    let hours: Hours = minutes.into();
    assert_eq!(hours.0, 1);
}
