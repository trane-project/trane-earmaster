use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::interval_comparison");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Interval Comparison".to_string(),
        directory_name: "interval_comparison".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - Compare the perfect intervals - common 1st tone.
            EarMasterLesson::new("1.2", "Perfect 4th & Octave  - Ascending", vec![]),
            EarMasterLesson::new("1.3", "Perfect 4th & Octave  - Descending", vec![]),
            EarMasterLesson::new("1.4", "Perfect 4th & Octave  - Harmonic", vec![]),
            EarMasterLesson::new("1.5", "Perfect 5th & Octave  - Ascending", vec![]),
            EarMasterLesson::new("1.6", "Perfect 5th & Octave  - Descending", vec![]),
            EarMasterLesson::new("1.7", "Perfect 5th & Octave  - Harmonic", vec![]),
            EarMasterLesson::new(
                "1.8",
                "Perfect 4th & Perfect 5th  - Ascending",
                vec!["1.2", "1.5"],
            ),
            EarMasterLesson::new(
                "1.9",
                "Perfect 4th & Perfect 5th  - Descending",
                vec!["1.3", "1.6"],
            ),
            EarMasterLesson::new(
                "1.10",
                "Perfect 4th & Perfect 5th  - Harmonic",
                vec!["1.4", "1.7"],
            ),
            EarMasterLesson::new(
                "1.11",
                "Test: Perfect intervals with a common 1st tone",
                vec!["1.8", "1.9", "1.10"],
            ),
            // Unit 2 - Compare the imperfect consonant intervals - common 1st tone.
            EarMasterLesson::new("2.2", "Major 3rd & Minor 6th - Ascending", vec!["1.11"]),
            EarMasterLesson::new("2.3", "Major 3rd & Minor 6th - Descending", vec!["1.11"]),
            EarMasterLesson::new("2.4", "Major 3rd & Minor 6th - Harmonic", vec!["1.11"]),
            EarMasterLesson::new("2.5", "Minor 3rd & Major 6th - Ascending", vec!["1.11"]),
            EarMasterLesson::new("2.6", "Minor 3rd & Major 6th - Descending", vec!["1.11"]),
            EarMasterLesson::new("2.7", "Minor 3rd & Major 6th - Harmonic", vec!["1.11"]),
            EarMasterLesson::new(
                "2.8",
                "Minor 3rd & Major 3rd - Ascending",
                vec!["2.2", "2.5"],
            ),
            EarMasterLesson::new(
                "2.9",
                "Minor 3rd & Major 3rd - Descending",
                vec!["2.3", "2.6"],
            ),
            EarMasterLesson::new(
                "2.10",
                "Minor 3rd & Major 3rd - Harmonic",
                vec!["2.4", "2.7"],
            ),
            EarMasterLesson::new(
                "2.11",
                "Minor 6th & Major 6th - Ascending",
                vec!["2.2", "2.5"],
            ),
            EarMasterLesson::new(
                "2.12",
                "Minor 6th & Major 6th - Descending",
                vec!["2.3", "2.6"],
            ),
            EarMasterLesson::new(
                "2.13",
                "Minor 6th & Major 6th - Harmonic",
                vec!["2.4", "2.7"],
            ),
            EarMasterLesson::new(
                "2.14",
                "Test: Imperfect consonant intervals with a common 1st tone",
                vec!["2.8", "2.9", "2.10", "2.11", "2.12", "2.13"],
            ),
            // Unit 3 - Compare the dissonant intervals - common 1st tone.
            EarMasterLesson::new("3.2", "Major 2nd & Minor 7th - Ascending", vec!["2.14"]),
            EarMasterLesson::new("3.3", "Major 2nd & Minor 7th - Descending", vec!["2.14"]),
            EarMasterLesson::new("3.4", "Major 2nd & Minor 7th - Harmonic", vec!["2.14"]),
            EarMasterLesson::new("3.5", "Minor 2nd & Major 7th - Ascending", vec!["2.14"]),
            EarMasterLesson::new("3.6", "Minor 2nd & Major 7th - Descending", vec!["2.14"]),
            EarMasterLesson::new("3.7", "Minor 2nd & Major 7th - Harmonic", vec!["2.14"]),
            EarMasterLesson::new(
                "3.8",
                "Major 2nd, Dim 5th & Minor 7th - Ascending",
                vec!["3.2", "3.5"],
            ),
            EarMasterLesson::new(
                "3.9",
                "Major 2nd, Dim 5th & Minor 7th - Descending",
                vec!["3.3", "3.6"],
            ),
            EarMasterLesson::new(
                "3.10",
                "Major 2nd, Dim 5th & Minor 7th - Harmonic",
                vec!["3.4", "3.7"],
            ),
            EarMasterLesson::new(
                "3.11",
                "Minor 2nd, Dim 5th & Major 7th - Ascending",
                vec!["3.2", "3.5"],
            ),
            EarMasterLesson::new(
                "3.12",
                "Minor 2nd, Dim 5th & Major 7th - Descending",
                vec!["3.3", "3.6"],
            ),
            EarMasterLesson::new(
                "3.13",
                "Minor 2nd, Dim 5th & Major 7th - Harmonic",
                vec!["3.4", "3.7"],
            ),
            EarMasterLesson::new(
                "3.14",
                "Minor 2nd & Major 2nd - Ascending",
                vec!["3.8", "3.11"],
            ),
            EarMasterLesson::new(
                "3.15",
                "Minor 2nd & Major 2nd - Descending",
                vec!["3.9", "3.12"],
            ),
            EarMasterLesson::new(
                "3.16",
                "Minor 2nd & Major 2nd - Harmonic",
                vec!["3.10", "3.13"],
            ),
            EarMasterLesson::new(
                "3.17",
                "Minor 7th & Major 7th - Ascending",
                vec!["3.8", "3.11"],
            ),
            EarMasterLesson::new(
                "3.18",
                "Minor 7th & Major 7th - Descending",
                vec!["3.9", "3.12"],
            ),
            EarMasterLesson::new(
                "3.19",
                "Minor 7th & Major 7th - Harmonic",
                vec!["3.10", "3.13"],
            ),
            EarMasterLesson::new(
                "3.20",
                "Test: Dissonant intervals with a common 1st tone",
                vec!["3.14", "3.15", "3.16", "3.17", "3.18", "3.19"],
            ),
            // Unit 4 - Compare all simple intervals - common 1st tone.
            EarMasterLesson::new(
                "4.1",
                "All intervals from Minor 6th to Major 7th - Ascending",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.2",
                "All intervals from Minor 6th to Major 7th - Descending",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.3",
                "All intervals from Minor 6th to Major 7th - Harmonic",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.4",
                "All intervals from Perfect 5th to Octave - Ascending",
                vec!["4.1"],
            ),
            EarMasterLesson::new(
                "4.5",
                "All intervals from Perfect 5th to Octave - Descending",
                vec!["4.2"],
            ),
            EarMasterLesson::new(
                "4.6",
                "All intervals from Perfect 5th to Octave - Harmonic",
                vec!["4.3"],
            ),
            EarMasterLesson::new(
                "4.7",
                "All intervals from Minor 2nd to Major 3rd - Ascending",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.8",
                "All intervals from Minor 2nd to Major 3rd - Descending",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.9",
                "All intervals from Minor 2nd to Major 3rd - Harmonic",
                vec!["3.20"],
            ),
            EarMasterLesson::new(
                "4.10",
                "All intervals from Minor 2nd to Dim 5th - Ascending",
                vec!["4.7"],
            ),
            EarMasterLesson::new(
                "4.11",
                "All intervals from Minor 2nd to Dim 5th - Descending",
                vec!["4.8"],
            ),
            EarMasterLesson::new(
                "4.12",
                "All intervals from Minor 2nd to Dim 5th - Harmonic",
                vec!["4.9"],
            ),
            EarMasterLesson::new(
                "4.13",
                "Test: Simple intervals with a common 1st tone",
                vec!["4.4", "4.5", "4.6", "4.10", "4.11", "4.12"],
            ),
            // Unit 5 - Compare compound intervals up to 2 octaves - common 1st tone.
            EarMasterLesson::new(
                "5.2",
                "Perfect 11th, Perfect 12th & Two Octaves - Ascending",
                vec!["4.13"],
            ),
            EarMasterLesson::new(
                "5.3",
                "Perfect 11th, Perfect 12th & Two Octaves - Descending",
                vec!["4.13"],
            ),
            EarMasterLesson::new(
                "5.4",
                "Perfect 11th, Perfect 12th & Two Octaves - Harmonic",
                vec!["4.13"],
            ),
            EarMasterLesson::new(
                "5.5",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Ascending",
                vec!["5.2"],
            ),
            EarMasterLesson::new(
                "5.6",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Descending",
                vec!["5.3"],
            ),
            EarMasterLesson::new(
                "5.7",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Harmonic",
                vec!["5.4"],
            ),
            EarMasterLesson::new(
                "5.8",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Ascending",
                vec!["5.5"],
            ),
            EarMasterLesson::new(
                "5.9",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Descending",
                vec!["5.6"],
            ),
            EarMasterLesson::new(
                "5.10",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Harmonic",
                vec!["5.7"],
            ),
            EarMasterLesson::new(
                "5.11",
                "Test: Compound intervals with a common 1st tone",
                vec!["5.8", "5.9", "5.10"],
            ),
            // Unit 6: Compare the perfect intervals - common 1st or 2nd tone.
            EarMasterLesson::new("6.1", "Perfect 4th & Octave  - Ascending", vec!["5.11"]),
            EarMasterLesson::new("6.2", "Perfect 4th & Octave  - Descending", vec!["5.11"]),
            EarMasterLesson::new("6.3", "Perfect 4th & Octave  - Harmonic", vec!["5.11"]),
            EarMasterLesson::new("6.4", "Perfect 5th & Octave  - Ascending", vec!["5.11"]),
            EarMasterLesson::new("6.5", "Perfect 5th & Octave  - Descending", vec!["5.11"]),
            EarMasterLesson::new("6.6", "Perfect 5th & Octave  - Harmonic", vec!["5.11"]),
            EarMasterLesson::new(
                "6.7",
                "Perfect 4th & Perfect 5th - Ascending",
                vec!["6.1", "6.4"],
            ),
            EarMasterLesson::new(
                "6.8",
                "Perfect 4th & Perfect 5th - Descending",
                vec!["6.2", "6.5"],
            ),
            EarMasterLesson::new(
                "6.9",
                "Perfect 4th & Perfect 5th - Harmonic",
                vec!["6.3", "6.6"],
            ),
            EarMasterLesson::new(
                "6.10",
                "Test: Perfect intervals with a common 1st or 2nd tone",
                vec!["6.7", "6.8", "6.9"],
            ),
            // Unit 7: Compare the imperfect consonant intervals - common 1st or 2nd tone.
            EarMasterLesson::new("7.1", "Major 3rd & Minor 6th - Ascending", vec!["6.10"]),
            EarMasterLesson::new("7.2", "Major 3rd & Minor 6th - Descending", vec!["6.10"]),
            EarMasterLesson::new("7.3", "Major 3rd & Minor 6th - Harmonic", vec!["6.10"]),
            EarMasterLesson::new("7.4", "Minor 3rd & Major 6th - Ascending", vec!["6.10"]),
            EarMasterLesson::new("7.5", "Minor 3rd & Major 6th - Descending", vec!["6.10"]),
            EarMasterLesson::new("7.6", "Minor 3rd & Major 6th - Harmonic", vec!["6.10"]),
            EarMasterLesson::new(
                "7.7",
                "Major 3rd & Minor 3rd - Ascending",
                vec!["7.1", "7.4"],
            ),
            EarMasterLesson::new(
                "7.8",
                "Major 3rd & Minor 3rd - Descending",
                vec!["7.2", "7.5"],
            ),
            EarMasterLesson::new(
                "7.9",
                "Major 3rd & Minor 3rd - Harmonic",
                vec!["7.3", "7.6"],
            ),
            EarMasterLesson::new(
                "7.10",
                "Minor 6th & Major 6th - Ascending",
                vec!["7.1", "7.4"],
            ),
            EarMasterLesson::new(
                "7.11",
                "Minor 6th & Major 6th - Descending",
                vec!["7.2", "7.5"],
            ),
            EarMasterLesson::new(
                "7.12",
                "Minor 6th & Major 6th - Harmonic",
                vec!["7.3", "7.6"],
            ),
            EarMasterLesson::new(
                "7.13",
                "Test: Imperfect consonant intervals with a common 1st or 2nd tone",
                vec!["7.7", "7.8", "7.9", "7.10", "7.11", "7.12"],
            ),
            // Unit 8 - Compare all the dissonant intervals - common 1st or 2nd tone.
            EarMasterLesson::new("8.1", "Major 2nd & Minor 7th - Ascending", vec!["7.13"]),
            EarMasterLesson::new("8.2", "Major 2nd & Minor 7th - Descending", vec!["7.13"]),
            EarMasterLesson::new("8.3", "Major 2nd & Minor 7th - Harmonic", vec!["7.13"]),
            EarMasterLesson::new("8.4", "Minor 2nd & Major 7th - Ascending", vec!["7.13"]),
            EarMasterLesson::new("8.5", "Minor 2nd & Major 7th - Descending", vec!["7.13"]),
            EarMasterLesson::new("8.6", "Minor 2nd & Major 7th - Harmonic", vec!["7.13"]),
            EarMasterLesson::new(
                "8.7",
                "Major 2nd, Dim 5th & Minor 7th - Ascending",
                vec!["8.1", "8.4"],
            ),
            EarMasterLesson::new(
                "8.8",
                "Major 2nd, Dim 5th & Minor 7th - Descending",
                vec!["8.2", "8.5"],
            ),
            EarMasterLesson::new(
                "8.9",
                "Major 2nd, Dim 5th & Minor 7th - Harmonic",
                vec!["8.3", "8.6"],
            ),
            EarMasterLesson::new(
                "8.10",
                "Minor 2nd, Dim 5th & Major 7th - Ascending",
                vec!["8.1", "8.4"],
            ),
            EarMasterLesson::new(
                "8.11",
                "Minor 2nd, Dim 5th & Major 7th - Descending",
                vec!["8.2", "8.5"],
            ),
            EarMasterLesson::new(
                "8.12",
                "Minor 2nd, Dim 5th & Major 7th - Harmonic",
                vec!["8.3", "8.6"],
            ),
            EarMasterLesson::new(
                "8.13",
                "Minor 2nd & Major 2nd - Ascending",
                vec!["8.7", "8.10"],
            ),
            EarMasterLesson::new(
                "8.14",
                "Minor 2nd & Major 2nd - Descending",
                vec!["8.8", "8.11"],
            ),
            EarMasterLesson::new(
                "8.15",
                "Minor 2nd & Major 2nd - Harmonic",
                vec!["8.9", "8.12"],
            ),
            EarMasterLesson::new(
                "8.16",
                "Minor 7th & Major 7th - Ascending",
                vec!["8.7", "8.10"],
            ),
            EarMasterLesson::new(
                "8.17",
                "Minor 7th & Major 7th - Descending",
                vec!["8.8", "8.11"],
            ),
            EarMasterLesson::new(
                "8.18",
                "Minor 7th & Major 7th - Harmonic",
                vec!["8.9", "8.12"],
            ),
            EarMasterLesson::new(
                "8.19",
                "Test: Dissonant intervals with a common 1st or 2nd tone",
                vec!["8.13", "8.14", "8.15", "8.16", "8.17", "8.18"],
            ),
            // Unit 9 - Compare all simple intervals - common 1st or 2nd tone.
            EarMasterLesson::new(
                "9.1",
                "All intervals from Minor 6th to Major 7th - Ascending",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.2",
                "All intervals from Minor 6th to Major 7th - Descending",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.3",
                "All intervals from Minor 6th to Major 7th - Harmonic",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.4",
                "All intervals from Perfect 5th to Octave - Ascending",
                vec!["9.1"],
            ),
            EarMasterLesson::new(
                "9.5",
                "All intervals from Perfect 5th to Octave - Descending",
                vec!["9.2"],
            ),
            EarMasterLesson::new(
                "9.6",
                "All intervals from Perfect 5th to Octave - Harmonic",
                vec!["9.3"],
            ),
            EarMasterLesson::new(
                "9.7",
                "All intervals from Minor 2nd to Major 3rd - Ascending",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.8",
                "All intervals from Minor 2nd to Major 3rd - Descending",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.9",
                "All intervals from Minor 2nd to Major 3rd - Harmonic",
                vec!["8.19"],
            ),
            EarMasterLesson::new(
                "9.10",
                "All intervals from Minor 2nd to Dim 5th - Ascending",
                vec!["9.7"],
            ),
            EarMasterLesson::new(
                "9.11",
                "All intervals from Minor 2nd to Dim 5th - Descending",
                vec!["9.8"],
            ),
            EarMasterLesson::new(
                "9.12",
                "All intervals from Minor 2nd to Dim 5th - Harmonic",
                vec!["9.9"],
            ),
            EarMasterLesson::new(
                "9.13",
                "Test: Simple intervals with a common 1st tone",
                vec!["9.4", "9.5", "9.6", "9.10", "9.11", "9.12"],
            ),
            // Unit 10 - Compare compound intervals up to 2 octaves - common 1st tone.
            EarMasterLesson::new(
                "10.1",
                "Perfect 11th, Perfect 12th & Two Octaves - Ascending",
                vec!["9.13"],
            ),
            EarMasterLesson::new(
                "10.2",
                "Perfect 11th, Perfect 12th & Two Octaves - Descending",
                vec!["9.13"],
            ),
            EarMasterLesson::new(
                "10.3",
                "Perfect 11th, Perfect 12th & Two Octaves - Harmonic",
                vec!["9.13"],
            ),
            EarMasterLesson::new(
                "10.4",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Ascending",
                vec!["10.1"],
            ),
            EarMasterLesson::new(
                "10.5",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Descending",
                vec!["10.2"],
            ),
            EarMasterLesson::new(
                "10.6",
                "Minor 10th, Major 10th, Minor 13th & Major 13th - Harmonic",
                vec!["10.3"],
            ),
            EarMasterLesson::new(
                "10.7",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Ascending",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "10.8",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Descending",
                vec!["10.5"],
            ),
            EarMasterLesson::new(
                "10.9",
                "Minor 9th, Major 9th, Perfect 12th, Minor 14th & Major 14th - Harmonic",
                vec!["10.6"],
            ),
            EarMasterLesson::new(
                "10.10",
                "Test: Compound intervals with a common 1st tone",
                vec!["10.7", "10.8", "10.9"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
