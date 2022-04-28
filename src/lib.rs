//! # Keynergy
//!
//! Keynergy is a powerful, efficient, and extensible library for
//! keyboard layout analysis.
//!
//! ## Features
//!
//! Keynergy allows you to:
//! - serialize and deserialize layouts, keyboards, metrics, text
//! data, and more using [Serde](https://docs.rs/serde)
//! - get the frequencies of characters, bigrams, trigrams, and skipgrams in a text
//! - declaratively write custom metrics using
//! [Ketos](https://docs.rs/ketos)
//! - prebake almost all analysis data before the layout is even known, making Keynergy
//! extremely efficient
//!
//! ## Example
//! ```rust
//! use keynergy::{Keyboard, Layout, Keys, Direction, Pos};
//!
//! let mut qwerty = Keys::qwerty();
//! assert_eq!(qwerty.pos_key(Pos::new(0, 0)), Some(&'q'));
//! assert_eq!(qwerty.pos_key(Pos::new(0, 1)), Some(&'a'));
//!
//! // Easily swap keys.
//! qwerty.swap(Pos::new(0,0), Pos::new(0, 1));
//! assert_eq!(qwerty.pos_key(Pos::new(0,0)), Some(&'a'));
//!
//! // Provides constants for our 10 human fingers.
//! use keynergy::fingers::*;
//!
//! // Get the direction between two fingers.
//! assert_eq!(Direction::from(LI, LM), Direction::Outward);
//! assert_eq!(Direction::from(LM, LI), Direction::Inward);
//! assert_eq!(Direction::from(LI, LI), Direction::None);
//! assert_eq!(Direction::from(LI, RM), Direction::None);
//!
//! // Can also be written this way.
//! assert_eq!(LI.dir_to(LM), Direction::Outward);
//! ```
//!
//! ## Analysis
//! Analysis is done in four steps.
//!
//! First, the `Analyzer` struct is created using `TextData` and a `MetricList`.
//! ```rust
//! // Requires the "analysis" feature.
//! use keynergy::analysis::{MetricList, Metric, Analyzer, InputType};
//! use keynergy::TextData;
//! let mut metrics = MetricList::new();
//! metrics.bigrams.insert(
//!   "SFB".to_string(),
//!   Metric {
//!     function: "sfb?".to_string(),
//!     input: InputType::Bigram,
//!    },
//! );
//! let td = TextData::from("decided");
//! let mut analyzer = Analyzer::with(metrics, &td);
//! ```
//! Next, the metric code is executed.
//! ```rust
//! # use keynergy::analysis::{MetricList, Metric, Analyzer, InputType};
//! # use keynergy::TextData;
//! # let mut metrics = MetricList::new();
//! # metrics.bigrams.insert(
//! #   "SFB".to_string(),
//! #   Metric {
//! #     function: "sfb?".to_string(),
//! #     input: InputType::Bigram,
//! #    },
//! # );
//! # let td = TextData::from("decided");
//! # let mut analyzer = Analyzer::with(metrics, &td);
//! analyzer
//!   .interpreter
//!   .run_code(
//!     r#"
//!       (define (sfb? a b)
//!               (= (finger a)
//!                  (finger b)))
//!     "#, None).unwrap();
//! ```
//! Next, the metrics are calculated for a keyboard.
//! No layout is needed in this stage, but almost everything
//! is precalculated here.
//! ```rust
//! # use keynergy::analysis::{MetricList, Metric, InputType, Analyzer};
//! # use keynergy::TextData;
//! # let mut metrics = MetricList::new();
//! # metrics.bigrams.insert(
//! #  "SFB".to_string(),
//! #  Metric {
//! #    function: "sfb?".to_string(),
//! #    input: InputType::Bigram,
//! #   },
//! # );
//! # let td = TextData::from("decided");
//! # let mut analyzer = Analyzer::with(metrics, &td);
//! # analyzer
//! #   .interpreter
//! #   .run_code(
//! #     r#"
//! #       (define (sfb? a b)
//! #               (= (finger a)
//! #                  (finger b)))
//! #     "#, None).unwrap();
//! use keynergy::Keyboard;
//! let matrix = Keyboard::matrix();
//! // Pushes to the `keyboard_stats` field of the analyzer.
//! analyzer.calculate_metrics(&matrix);
//! ```
//!
//! Finally, the actual layout can be analyzed.
//! ```
//! # use keynergy::analysis::{MetricList, Metric, InputType, Analyzer};
//! # use keynergy::TextData;
//! # let mut metrics = MetricList::new();
//! # metrics.bigrams.insert(
//! #   "SFB".to_string(),
//! #    Metric {
//! #      function: "sfb?".to_string(),
//! #      input: InputType::Bigram,
//! #     },
//! # );
//! # let td = TextData::from("decided");
//! # println!("{:?}", td);
//! # let mut analyzer = Analyzer::with(metrics, &td);
//! # analyzer
//! #   .interpreter
//! #   .run_code(
//! #     r#"
//! #       (define (sfb? a b)
//! #          (= (finger a)
//! #             (finger b)))
//! #     "#, None).unwrap();
//! # use keynergy::Keyboard;
//! # let matrix = Keyboard::matrix();
//! # analyzer.calculate_metrics(&matrix);
//! # println!("{:?}",analyzer.keyboard_stats);
//! use keynergy::Keys;
//! let qwerty = Keys::qwerty();
//! let result = analyzer.analyze_keys(matrix, qwerty).unwrap();
//!
//! use keynergy::analysis::MetricTotal;
//! println!("{:?}", result);
//! assert_eq!(*result.get("SFB").unwrap(),
//! MetricTotal::Count(4));
//! ```

//#![warn(missing_docs)]

#[cfg(feature = "analysis")]
pub mod analysis;
pub mod direction;
pub mod finger;
pub mod fingers;
pub mod keyboard;
pub mod layout;
pub mod pos;
pub mod textdata;
#[cfg(feature = "analysis")]
pub use analysis::analyzer::Analyzer;
pub use direction::Direction;
pub use finger::{Finger, FingerKind, Hand};
pub use keyboard::Keyboard;
pub use layout::{Formats, Keys, Layout};
pub use pos::{Pos, PosGroup, PosPair};
pub use textdata::TextData;
