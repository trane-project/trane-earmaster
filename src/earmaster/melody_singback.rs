use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::melody_singback");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Melody Singback".to_string(),
        directory_name: "earmaster_melody_singback".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - 3 to 4 tones from the C major scale
            EarMasterLesson::new("1.1", "Do, Re - 3 tones", vec![]),
            EarMasterLesson::new("1.2", "Do, Re, Mi", vec!["1.1"]),
            EarMasterLesson::new("1.3", "Do, Re, Mi, Fa", vec!["1.2"]),
            EarMasterLesson::new("1.4", "Do, Re, Mi, Fa, So", vec!["1.3"]),
            EarMasterLesson::new("1.5", "Do, Re, Mi, Fa, So, La", vec!["1.4"]),
            EarMasterLesson::new("1.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["1.5"]),
            EarMasterLesson::new("1.7", "Do, Re - 4 tones", vec!["1.1"]),
            EarMasterLesson::new("1.8", "Do, Re, Mi", vec!["1.7"]),
            EarMasterLesson::new("1.9", "Do, Re, Mi, Fa", vec!["1.8"]),
            EarMasterLesson::new("1.10", "Do, Re, Mi, Fa, So", vec!["1.9"]),
            EarMasterLesson::new("1.11", "Do, Re, Mi, Fa, So, La", vec!["1.10"]),
            EarMasterLesson::new("1.12", "Do, Re, Mi, Fa, So, La, Ti", vec!["1.11"]),
            // Unit 2 - 5 to 6 tones from the C major scale
            EarMasterLesson::new("2.1", "Do, Re - 5 tones", vec!["1.7"]),
            EarMasterLesson::new("2.2", "Do, Re, Mi", vec!["2.1"]),
            EarMasterLesson::new("2.3", "Do, Re, Mi, Fa", vec!["2.2"]),
            EarMasterLesson::new("2.4", "Do, Re, Mi, Fa, So", vec!["2.3"]),
            EarMasterLesson::new("2.5", "Do, Re, Mi, Fa, So, La", vec!["2.4"]),
            EarMasterLesson::new("2.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["2.5"]),
            EarMasterLesson::new("2.7", "Do, Re - 6 tones", vec!["2.1"]),
            EarMasterLesson::new("2.8", "Do, Re, Mi", vec!["2.7"]),
            EarMasterLesson::new("2.9", "Do, Re, Mi, Fa", vec!["2.8"]),
            EarMasterLesson::new("2.10", "Do, Re, Mi, Fa, So", vec!["2.9"]),
            EarMasterLesson::new("2.11", "Do, Re, Mi, Fa, So, La", vec!["2.10"]),
            EarMasterLesson::new("2.12", "Do, Re, Mi, Fa, So, La, Ti", vec!["2.11"]),
            // Unit 3 - 7 to 8 tones from the C major scale
            EarMasterLesson::new("3.1", "Do, Re, Mi - 7 tones", vec!["2.7"]),
            EarMasterLesson::new("3.2", "Do, Re, Mi, Fa", vec!["3.1"]),
            EarMasterLesson::new("3.3", "Do, Re, Mi, Fa, So", vec!["3.2"]),
            EarMasterLesson::new("3.4", "Do, Re, Mi, Fa, So, La", vec!["3.3"]),
            EarMasterLesson::new("3.5", "Do, Re, Mi, Fa, So, La, Ti", vec!["3.4"]),
            EarMasterLesson::new("3.6", "Do, Re, Mi - 8 tones", vec!["3.1"]),
            EarMasterLesson::new("3.7", "Do, Re, Mi, Fa", vec!["3.6"]),
            EarMasterLesson::new("3.8", "Do, Re, Mi, Fa, So", vec!["3.7"]),
            EarMasterLesson::new("3.9", "Do, Re, Mi, Fa, So, La", vec!["3.8"]),
            EarMasterLesson::new("3.10", "Do, Re, Mi, Fa, So, La, Ti", vec!["3.9"]),
            // Unit 4 - Melodies in C major with rhythm. 4/4
            EarMasterLesson::new("4.1", "Do, Re - 1 bar", vec!["3.10"]),
            EarMasterLesson::new("4.2", "Do, Re, Mi", vec!["4.1"]),
            EarMasterLesson::new("4.3", "Do, Re, Mi, Fa", vec!["4.2"]),
            EarMasterLesson::new("4.4", "Do, Re, Mi, Fa, So", vec!["4.3"]),
            EarMasterLesson::new("4.5", "Do, Re, Mi, Fa, So, La", vec!["4.4"]),
            EarMasterLesson::new("4.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["4.5"]),
            EarMasterLesson::new("4.7", "Do, Re, Mi - 2 bars", vec!["4.2"]),
            EarMasterLesson::new("4.8", "Do, Re, Mi, Fa", vec!["4.7"]),
            EarMasterLesson::new("4.9", "Do, Re, Mi, Fa, So", vec!["4.8"]),
            EarMasterLesson::new("4.10", "Do, Re, Mi, Fa, So, La", vec!["4.9"]),
            EarMasterLesson::new("4.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["4.10"]),
            // Unit 5 - Melodies in C major with rhythm. 3/4
            EarMasterLesson::new("5.1", "Do, Re - 1 bar", vec!["3.10"]),
            EarMasterLesson::new("5.2", "Do, Re, Mi", vec!["5.1"]),
            EarMasterLesson::new("5.3", "Do, Re, Mi, Fa", vec!["5.2"]),
            EarMasterLesson::new("5.4", "Do, Re, Mi, Fa, So", vec!["5.3"]),
            EarMasterLesson::new("5.5", "Do, Re, Mi, Fa, So, La", vec!["5.4"]),
            EarMasterLesson::new("5.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["5.5"]),
            EarMasterLesson::new("5.7", "Do, Re, Mi - 2 bars", vec!["5.2"]),
            EarMasterLesson::new("5.8", "Do, Re, Mi, Fa", vec!["5.7"]),
            EarMasterLesson::new("5.9", "Do, Re, Mi, Fa, So", vec!["5.8"]),
            EarMasterLesson::new("5.10", "Do, Re, Mi, Fa, So, La", vec!["5.9"]),
            EarMasterLesson::new("5.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["5.10"]),
            // Unit 6 - Adding 8th Notes. 4/4
            EarMasterLesson::new("6.1", "Do, Re - 1 bar", vec!["4.11"]),
            EarMasterLesson::new("6.2", "Do, Re, Mi", vec!["6.1"]),
            EarMasterLesson::new("6.3", "Do, Re, Mi, Fa", vec!["6.2"]),
            EarMasterLesson::new("6.4", "Do, Re, Mi, Fa, So", vec!["6.3"]),
            EarMasterLesson::new("6.5", "Do, Re, Mi, Fa, So, La", vec!["6.4"]),
            EarMasterLesson::new("6.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["6.5"]),
            EarMasterLesson::new("6.7", "Do, Re, Mi - 2 bars", vec!["6.2"]),
            EarMasterLesson::new("6.8", "Do, Re, Mi, Fa", vec!["6.7"]),
            EarMasterLesson::new("6.9", "Do, Re, Mi, Fa, So", vec!["6.8"]),
            EarMasterLesson::new("6.10", "Do, Re, Mi, Fa, So, La", vec!["6.9"]),
            EarMasterLesson::new("6.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["6.10"]),
            // Unit 7 - Adding 8th Notes. 3/4
            EarMasterLesson::new("7.1", "Do, Re - 1 bar", vec!["5.11"]),
            EarMasterLesson::new("7.2", "Do, Re, Mi", vec!["7.1"]),
            EarMasterLesson::new("7.3", "Do, Re, Mi, Fa", vec!["7.2"]),
            EarMasterLesson::new("7.4", "Do, Re, Mi, Fa, So", vec!["7.3"]),
            EarMasterLesson::new("7.5", "Do, Re, Mi, Fa, So, La", vec!["7.4"]),
            EarMasterLesson::new("7.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["7.5"]),
            EarMasterLesson::new("7.7", "Do, Re, Mi - 2 bars", vec!["7.2"]),
            EarMasterLesson::new("7.8", "Do, Re, Mi, Fa", vec!["7.7"]),
            EarMasterLesson::new("7.9", "Do, Re, Mi, Fa, So", vec!["7.8"]),
            EarMasterLesson::new("7.10", "Do, Re, Mi, Fa, So, La", vec!["7.9"]),
            EarMasterLesson::new("7.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["7.10"]),
            // Unit 8 - Around the Circle of Fifths. 4/4
            EarMasterLesson::new("8.1", "Do, Re - 1 bar", vec!["6.11"]),
            EarMasterLesson::new("8.2", "Do, Re, Mi", vec!["8.1"]),
            EarMasterLesson::new("8.3", "Do, Re, Mi, Fa", vec!["8.2"]),
            EarMasterLesson::new("8.4", "Do, Re, Mi, Fa, So", vec!["8.3"]),
            EarMasterLesson::new("8.5", "Do, Re, Mi, Fa, So, La", vec!["8.4"]),
            EarMasterLesson::new("8.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["8.5"]),
            EarMasterLesson::new("8.7", "Do, Re, Mi - 2 bars", vec!["8.2"]),
            EarMasterLesson::new("8.8", "Do, Re, Mi, Fa", vec!["8.7"]),
            EarMasterLesson::new("8.9", "Do, Re, Mi, Fa, So", vec!["8.8"]),
            EarMasterLesson::new("8.10", "Do, Re, Mi, Fa, So, La", vec!["8.9"]),
            EarMasterLesson::new("8.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["8.10"]),
            // Unit 9 - Around the Circle of Fifths. 3/4
            EarMasterLesson::new("9.1", "Do, Re - 1 bar", vec!["7.11"]),
            EarMasterLesson::new("9.2", "Do, Re, Mi", vec!["9.1"]),
            EarMasterLesson::new("9.3", "Do, Re, Mi, Fa", vec!["9.2"]),
            EarMasterLesson::new("9.4", "Do, Re, Mi, Fa, So", vec!["9.3"]),
            EarMasterLesson::new("9.5", "Do, Re, Mi, Fa, So, La", vec!["9.4"]),
            EarMasterLesson::new("9.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["9.5"]),
            EarMasterLesson::new("9.7", "Do, Re, Mi - 2 bars", vec!["9.2"]),
            EarMasterLesson::new("9.8", "Do, Re, Mi, Fa", vec!["9.7"]),
            EarMasterLesson::new("9.9", "Do, Re, Mi, Fa, So", vec!["9.8"]),
            EarMasterLesson::new("9.10", "Do, Re, Mi, Fa, So, La", vec!["9.9"]),
            EarMasterLesson::new("9.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["9.10"]),
            // Unit 10 - Around the Circle of Fifths. 4/4 with 8th Notes
            EarMasterLesson::new("10.1", "Do, Re - 1 bar", vec!["8.11"]),
            EarMasterLesson::new("10.2", "Do, Re, Mi", vec!["10.1"]),
            EarMasterLesson::new("10.3", "Do, Re, Mi, Fa", vec!["10.2"]),
            EarMasterLesson::new("10.4", "Do, Re, Mi, Fa, So", vec!["10.3"]),
            EarMasterLesson::new("10.5", "Do, Re, Mi, Fa, So, La", vec!["10.4"]),
            EarMasterLesson::new("10.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["10.5"]),
            EarMasterLesson::new("10.7", "Do, Re, Mi - 2 bars", vec!["10.2"]),
            EarMasterLesson::new("10.8", "Do, Re, Mi, Fa", vec!["10.7"]),
            EarMasterLesson::new("10.9", "Do, Re, Mi, Fa, So", vec!["10.8"]),
            EarMasterLesson::new("10.10", "Do, Re, Mi, Fa, So, La", vec!["10.9"]),
            EarMasterLesson::new("10.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["10.10"]),
            // Unit 11 - Around the Circle of Fifths. 3/4 with 8th Notes
            EarMasterLesson::new("11.1", "Do, Re - 1 bar", vec!["9.11"]),
            EarMasterLesson::new("11.2", "Do, Re, Mi", vec!["11.1"]),
            EarMasterLesson::new("11.3", "Do, Re, Mi, Fa", vec!["11.2"]),
            EarMasterLesson::new("11.4", "Do, Re, Mi, Fa, So", vec!["11.3"]),
            EarMasterLesson::new("11.5", "Do, Re, Mi, Fa, So, La", vec!["11.4"]),
            EarMasterLesson::new("11.6", "Do, Re, Mi, Fa, So, La, Ti", vec!["11.5"]),
            EarMasterLesson::new("11.7", "Do, Re, Mi - 2 bars", vec!["11.2"]),
            EarMasterLesson::new("11.8", "Do, Re, Mi, Fa", vec!["11.7"]),
            EarMasterLesson::new("11.9", "Do, Re, Mi, Fa, So", vec!["11.8"]),
            EarMasterLesson::new("11.10", "Do, Re, Mi, Fa, So, La", vec!["11.9"]),
            EarMasterLesson::new("11.11", "Do, Re, Mi, Fa, So, La, Ti", vec!["11.10"]),
            // Unit 12 - Melodies in A minor. 4/4
            EarMasterLesson::new("12.1", "Do, Re - 1 bar", vec!["10.11"]),
            EarMasterLesson::new("12.2", "Do, Re, Me", vec!["12.1"]),
            EarMasterLesson::new("12.3", "Do, Re, Me, Fa", vec!["12.2"]),
            EarMasterLesson::new("12.4", "Do, Re, Me, Fa, So", vec!["12.3"]),
            EarMasterLesson::new("12.5", "Do, Re, Me, Fa, So, La", vec!["12.4"]),
            EarMasterLesson::new("12.6", "Do, Re, Me, Fa, So, La, Te", vec!["12.5"]),
            EarMasterLesson::new("12.7", "Do, Re, Me - 2 bars", vec!["12.2"]),
            EarMasterLesson::new("12.8", "Do, Re, Me, Fa", vec!["12.7"]),
            EarMasterLesson::new("12.9", "Do, Re, Me, Fa, So", vec!["12.8"]),
            EarMasterLesson::new("12.10", "Do, Re, Me, Fa, So, La", vec!["12.9"]),
            EarMasterLesson::new("12.11", "Do, Re, Me, Fa, So, La, Te", vec!["12.10"]),
            // Unit 13 - Melodies in A minor. 3/4
            EarMasterLesson::new("13.1", "Do, Re - 1 bar", vec!["11.11"]),
            EarMasterLesson::new("13.2", "Do, Re, Me", vec!["13.1"]),
            EarMasterLesson::new("13.3", "Do, Re, Me, Fa", vec!["13.2"]),
            EarMasterLesson::new("13.4", "Do, Re, Me, Fa, So", vec!["13.3"]),
            EarMasterLesson::new("13.5", "Do, Re, Me, Fa, So, La", vec!["13.4"]),
            EarMasterLesson::new("13.6", "Do, Re, Me, Fa, So, La, Te", vec!["13.5"]),
            EarMasterLesson::new("13.7", "Do, Re, Me - 2 bars", vec!["13.2"]),
            EarMasterLesson::new("13.8", "Do, Re, Me, Fa", vec!["13.7"]),
            EarMasterLesson::new("13.9", "Do, Re, Me, Fa, So", vec!["13.8"]),
            EarMasterLesson::new("13.10", "Do, Re, Me, Fa, So, La", vec!["13.9"]),
            EarMasterLesson::new("13.11", "Do, Re, Me, Fa, So, La, Te", vec!["13.10"]),
            // Unit 14 - Melodies in A minor. 4/4 with 8th Notes.
            EarMasterLesson::new("14.1", "Do, Re - 1 bar", vec!["12.11"]),
            EarMasterLesson::new("14.2", "Do, Re, Me", vec!["14.1"]),
            EarMasterLesson::new("14.3", "Do, Re, Me, Fa", vec!["14.2"]),
            EarMasterLesson::new("14.4", "Do, Re, Me, Fa, So", vec!["14.3"]),
            EarMasterLesson::new("14.5", "Do, Re, Me, Fa, So, La", vec!["14.4"]),
            EarMasterLesson::new("14.6", "Do, Re, Me, Fa, So, La, Te", vec!["14.5"]),
            EarMasterLesson::new("14.7", "Do, Re - 2 bars", vec!["14.1"]),
            EarMasterLesson::new("14.8", "Do, Re, Me", vec!["14.7"]),
            EarMasterLesson::new("14.9", "Do, Re, Me, Fa", vec!["14.8"]),
            EarMasterLesson::new("14.10", "Do, Re, Me, Fa, So", vec!["14.9"]),
            EarMasterLesson::new("14.11", "Do, Re, Me, Fa, So, La", vec!["14.10"]),
            EarMasterLesson::new("14.12", "Do, Re, Me, Fa, So, La, Te", vec!["14.11"]),
            // Unit 15 - Melodies in A minor. 3/4 with 8th Notes.
            EarMasterLesson::new("15.1", "Do, Re, Me - 1 bar", vec!["13.11"]),
            EarMasterLesson::new("15.2", "Do, Re, Me, Fa", vec!["15.1"]),
            EarMasterLesson::new("15.3", "Do, Re, Me, Fa, So", vec!["15.2"]),
            EarMasterLesson::new("15.4", "Do, Re, Me, Fa, So, La", vec!["15.3"]),
            EarMasterLesson::new("15.5", "Do, Re, Me, Fa, So, La, Te", vec!["15.4"]),
            EarMasterLesson::new("15.6", "Do, Re, Me - 2 bars", vec!["15.1"]),
            EarMasterLesson::new("15.7", "Do, Re, Me, Fa", vec!["15.6"]),
            EarMasterLesson::new("15.8", "Do, Re, Me, Fa, So", vec!["15.7"]),
            EarMasterLesson::new("15.9", "Do, Re, Me, Fa, So, La", vec!["15.8"]),
            EarMasterLesson::new("15.10", "Do, Re, Me, Fa, So, La, Te", vec!["15.9"]),
            // Unit 16 - All Minor Keys. 4/4 with 8th Notes.
            EarMasterLesson::new("16.1", "Do, Re, Me - 1 bar", vec!["14.12", "15.10"]),
            EarMasterLesson::new("16.2", "Do, Re, Me, Fa", vec!["16.1"]),
            EarMasterLesson::new("16.3", "Do, Re, Me, Fa, So", vec!["16.2"]),
            EarMasterLesson::new("16.4", "Do, Re, Me, Fa, So, La", vec!["16.3"]),
            EarMasterLesson::new("16.5", "Do, Re, Me, Fa, So, La, Te", vec!["16.4"]),
            EarMasterLesson::new("16.6", "Do, Re, Me - 2 bars", vec!["16.1"]),
            EarMasterLesson::new("16.7", "Do, Re, Me, Fa", vec!["16.6"]),
            EarMasterLesson::new("16.8", "Do, Re, Me, Fa, So", vec!["16.7"]),
            EarMasterLesson::new("16.9", "Do, Re, Me, Fa, So, La", vec!["16.8"]),
            EarMasterLesson::new("16.10", "Do, Re, Me, Fa, So, La, Te", vec!["16.9"]),
            // Unit 17 - Harmonic Minor. 4/4 with 8th Notes.
            EarMasterLesson::new("17.1", "A minor harmonic. 1 bar", vec!["16.10"]),
            EarMasterLesson::new("17.2", "A minor harmonic. 2 bars", vec!["17.1"]),
            EarMasterLesson::new("17.3", "A minor harmonic. 4 bars", vec!["17.2"]),
            EarMasterLesson::new("17.4", "Harmonic minor (all keys). 1 bar", vec!["17.3"]),
            EarMasterLesson::new("17.5", "Harmonic minor (all keys). 2 bars", vec!["17.4"]),
            EarMasterLesson::new("17.6", "Harmonic minor (all keys). 4 bars", vec!["17.5"]),
        ],
    };
    earmaster_course.course_builder()
}
