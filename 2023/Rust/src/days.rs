//! The handler for registering and getting the runners for each day.
//!
//! This contains all the code required to run all the runners for each day.  
//! In this module directly is the code required to register and get the runners for each day.
//!
//! Each day then has its own submodule containing its runner.
//!
//! # Examples
//!
//! An example of how to get and register a runner:
//! ```
//! use rust_aoc_2023::days;
//! use rust_aoc_2023::days::DayRunner;
//!
//! #[derive(Default)]
//! struct SomeRunner {}
//!
//! impl DayRunner for SomeRunner {}
//!
//! fn main() {
//!     days::init(false);
//!     let _ = days::register_day_runner_default::<SomeRunner>(19);
//!     let mut runner = days::get_day_runner(19).unwrap();
//!     assert!(runner.is_some());
//!     runner = days::get_day_runner(5).unwrap();
//!     assert!(runner.is_none());
//! }
//! ```
//!
//! This is how a runner is supposed to be implemented:
//! ```
//! # use std::error::Error;
//! #
//! # use rust_aoc_2023::days;
//! # use rust_aoc_2023::days::DayRunner;
//! #
//! #[derive(Default)]
//! struct SomeRunner {
//!     field: u32,
//! }
//!
//! impl DayRunner for SomeRunner {
//!     fn init(&mut self) -> Result<(), Box<dyn Error>> {
//!         assert_eq!(self.field, 0);
//!         // Some complex function to get common data.
//!         self.field = 91;
//!         Ok(())
//!     }
//!
//!     fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
//!         assert_eq!(self.field, 91);
//!         // Further logic, specific to part 1.
//!         let result = self.field * 3;
//!         Ok(Some(result.to_string()))
//!     }
//! }
//!
//! fn main() {
//!     days::init(false);
//!     let _ = days::register_day_runner_default::<SomeRunner>(7);
//!     let mut runner = days::get_day_runner(7).unwrap().unwrap();
//! #     runner.init().unwrap();
//! #     assert_eq!(runner.part1().unwrap().unwrap(), "273");
//! #     assert!(runner.part2().unwrap().is_none());
//! }
//! ```
//!
//! It is also possible to register a custom constructor, if this is desired for whatever reason:
//! ```
//! # use std::error::Error;
//! #
//! # use rust_aoc_2023::days;
//! # use rust_aoc_2023::days::DayRunner;
//! #
//! struct SomeRunner {
//!     field: u32,
//! }
//!
//! impl SomeRunner {
//!     pub fn new() -> SomeRunner {
//!         SomeRunner {
//!             field: 0,
//!         }
//!     }
//! }
//!
//! impl DayRunner for SomeRunner {
//!     fn init(&mut self) -> Result<(), Box<dyn Error>> {
//!         assert_eq!(self.field, 0);
//!         // Some complex function to get common data.
//!         self.field = 91;
//!         Ok(())
//!     }
//!
//! #     fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
//! #         assert_eq!(self.field, 91);
//! #         Ok(None)
//! #     }
//! }
//!
//! fn main() {
//!     days::init(false);
//!     let _ = days::register_day_runner_constructor(16, SomeRunner::new);
//!     let mut runner = days::get_day_runner(16).unwrap().unwrap();
//! #     runner.init().unwrap();
//! #     assert!(runner.part1().unwrap().is_none());
//! #     assert!(runner.part2().unwrap().is_none());
//! }

use std::error::Error;
use std::sync::{OnceLock, RwLock};

static DAY_RUNNERS: RwLock<Vec<Option<Box<dyn Fn() -> Box<dyn DayRunner> + Send + Sync>>>> =
    RwLock::new(Vec::new());

/// A runner solving a specific day of the [Advent of Code](https://adventofcode.com/).
///
/// Each struct implementing this is supposed to be responsible for solving a single of the advent of code.  
/// [Self::init] is supposed to initialize the variables required for both parts.  
/// [Self::part1] and [Self::part2] are then supposed to use these variables to solve the challenge.
///
/// The functions of this trait will be called like this when a day is executes:
/// ```
/// use std::error::Error;
///
/// use rust_aoc_2023::days::DayRunner;
///
/// struct ExampleRunner {
///     common: u32,
/// }
///
/// impl DayRunner for ExampleRunner {
///     fn init(&mut self) -> Result<(), Box<dyn Error>> {
///         // Some complex function to get common data.
///         self.common = 7;
///         Ok(())
///     }
///
///     fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
///         assert_eq!(self.common, 7);
///         // Additional functionality specific to part 1.
///         let result = self.common * 391;
///         Ok(Some(result.to_string()))
///     }
///
///     fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
///         assert_eq!(self.common, 7);
///         // Additional functionality specific to part 2.
///         Ok(None)
///     }
/// }
///
/// fn main() {
///     let mut runner = ExampleRunner {common: 0};
///     runner.init().unwrap();
///     let part1 = runner.part1().unwrap();
///     let part2 = runner.part2().unwrap();
/// #     assert_eq!(part1.unwrap(), "2737");
/// #     assert!(part2.is_none());
/// }
/// ```
pub trait DayRunner {
    /// The initialization function.
    ///
    /// This method is called to initialize common data used for both parts of the challenge.
    ///
    /// # Errors
    ///
    /// Depending on the implementation, this method is allowed to return arbitrary types of errors.
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// The method to solve part 1.
    ///
    /// This method is called after [Self::init], so everything calculated there is available.  
    /// Unlike [Self::init] however it is not allowed to mutate the runner.  
    ///
    /// This function should return the text representation of the part 1 solution for that day.  
    /// It is allowed to return `None`, first for not yet implemented parts,
    /// and second for the rare cases in which the output has to be directly printed to allow easy reading.
    ///
    /// # Errors
    ///
    /// Depending on the implementation, this method is allowed to return arbitrary types of errors.
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        Ok(None)
    }

    /// The method to solve part 2.
    ///
    /// This method is called after [Self::init], so everything calculated there is available.  
    /// Unlike [Self::init] however it is not allowed to mutate the runner.  
    ///
    /// This function should return the text representation of the part 1 solution for that day.  
    /// It is allowed to return `None`, first for not yet implemented parts,
    /// and second for the rare cases in which the output has to be directly printed to allow easy reading.
    ///
    /// # Errors
    ///
    /// Depending on the implementation, this method is allowed to return arbitrary types of errors.
    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        Ok(None)
    }
}

/// Initializes the internal runners list.
///
/// Initializes the interal data structure storing the constructors for each days runner.  
/// Has to be called before the first [get_day_runner] call.  
/// Currently also registers all the already implemented days.
///
/// Will automatically be called by the first [register_day_runner_constructor] or [register_day_runner_default] call.  
/// Calling this function a second time wont do anything.
///
/// TODO If possible, find a way for runners to register themselves.
///
/// # Panics
///
/// This function panics if the internal [RwLock] is poisoned.
///
/// # Examples
///
/// There is only realy one use-case for this function:
/// ```
/// use rust_aoc_2023::days;
///
/// days::init(false);
/// ```
pub fn init(register: bool) {
    if DAY_RUNNERS.read().unwrap().is_empty() {
        DAY_RUNNERS.write().unwrap().extend((0..25).map(|_| None));
        if register {
            register_day_runner_default::<day1::Day1Runner>(1);
            register_day_runner_default::<day2::Day2Runner>(2);
            register_day_runner_default::<day3::Day3Runner>(3);
            register_day_runner_default::<day4::Day4Runner>(4);
            register_day_runner_default::<day5::Day5Runner>(5);
            register_day_runner_default::<day6::Day6Runner>(6);
            register_day_runner_default::<day7::Day7Runner>(7);
            register_day_runner_default::<day8::Day8Runner>(8);
            register_day_runner_default::<day9::Day9Runner>(9);
            register_day_runner_default::<day10::Day10Runner>(10);
            register_day_runner_default::<day11::Day11Runner>(11);
            register_day_runner_default::<day12::Day12Runner>(12);
        }
    }
}

/// Get a runner for a specific day.
///
/// This function creates a new runner for the given day, if a constructor was registered for that day.  
/// Constructors can be registered using [register_day_runner_constructor] or [register_day_runner_default].  
///
/// If no constructor was registered this function returns `Ok(None)`.
///
/// # Errors
///
/// This function returns an error when the internal RwLock for that day is poisoned.  
/// An RwLock is poisoned whenever a writer panics while holding an exclusive lock.  
/// The failure will occur immediately after the lock has been acquired.
///
/// # Panics
///
/// This function panics when given a day that is either 0 or above 25.  
/// It will also panic if the internal [RwLock] is poisoned.  
/// This function also panics if the internal data structure isn't initialized, because [init] wasn't called.  
/// Note that calling [register_day_runner_constructor] or [register_day_runner_default] automatically runs [init].
///
/// # Examples
///
/// ```
/// # use rust_aoc_2023::days;
/// # use rust_aoc_2023::days::DayRunner;
/// #
/// # #[derive(Default)]
/// # struct SomeRunner {}
/// #
/// # impl DayRunner for SomeRunner {}
/// #
/// # fn main() {
///     days::init(false);
///     let _ = days::register_day_runner_default::<SomeRunner>(17);
///     let mut runner = days::get_day_runner(17).unwrap();
///     assert!(runner.is_some());
///     runner = days::get_day_runner(5).unwrap();
///     assert!(runner.is_none());
/// # }
/// ```
pub fn get_day_runner(day: u8) -> Result<Option<Box<dyn DayRunner>>, Box<dyn Error>> {
    if day == 0 {
        panic!("Trying to register runner for day 0.");
    } else if day > 25 {
        panic!("Trying to register runner for a day after the 25th.");
    }

    if DAY_RUNNERS.read()?.is_empty() {
        panic!("init has to be called for get_day_runner to work.");
    }

    match &DAY_RUNNERS.read()?[usize::from(day - 1)] {
        Some(func) => Ok(Some(func())),
        None => Ok(None),
    }
}

/// Register a runner constructor for a specific day.
///
/// This function registers the given constructor to be used to create a runner for the given day.  
/// To register a [Default] constructor use [register_day_runner_default].  
/// To get a runner use [get_day_runner].
///
/// # Panics
///
/// This function panics if given a day that is either 0 or above 25.  
/// And if the internal [RwLock] is poisoned.
///
/// # Examples
///
/// Using this function to register a `new` constructor:
/// ```
/// # use rust_aoc_2023::days;
/// # use rust_aoc_2023::days::DayRunner;
/// #
/// struct SomeRunner {}
///
/// impl SomeRunner {
///     pub fn new() -> SomeRunner {
///         SomeRunner {}
///     }
/// }
///
/// impl DayRunner for SomeRunner {}
///
/// fn main() {
///     days::init(false);
///     let _ = days::register_day_runner_constructor(6, SomeRunner::new);
///     let mut runner = days::get_day_runner(4).unwrap();
///     assert!(runner.is_none());
///     runner = days::get_day_runner(6).unwrap();
///     assert!(runner.is_some());
/// }
/// ```
pub fn register_day_runner_constructor<T, U>(day: u8, constructor: U) -> bool
where
    T: DayRunner + 'static,
    U: Fn() -> T + Send + Sync + 'static,
{
    if DAY_RUNNERS.read().unwrap().is_empty() {
        init(false);
    }

    if day == 0 {
        panic!("Trying to register runner for day 0.");
    } else if day > 25 {
        panic!("Trying to register runner for a day after the 25th.");
    }

    if DAY_RUNNERS.read().unwrap()[usize::from(day - 1)].is_some() {
        return false;
    }

    let cons: OnceLock<U> = OnceLock::new();
    let _ = cons.set(constructor);

    DAY_RUNNERS.write().unwrap()[usize::from(day - 1)] =
        Some(Box::new(move || wrap_constructor(&cons)));
    true
}

/// Register a default constructable day runner.
///
/// This function registers the [default](Default::default) constructor to be used to create a runner for the given day.  
/// To register a non-defaultable constructor use [register_day_runner_constructor].  
/// To get a runner use [get_day_runner].
///
/// # Panics
///
/// This function panics if given a day that is either 0 or above 25.  
/// And if the internal [RwLock] is poisoned.
///
/// # Examples
///
/// Using this function to register a [Default::default] constructor:
/// ```
/// # use rust_aoc_2023::days;
/// # use rust_aoc_2023::days::DayRunner;
/// #
/// #[derive(Default)]
/// struct SomeRunner {}
///
/// impl DayRunner for SomeRunner {}
///
/// fn main() {
///     days::init(false);
///     let _ = days::register_day_runner_default::<SomeRunner>(8);
///     let mut runner = days::get_day_runner(8).unwrap();
///     assert!(runner.is_some());
///     runner = days::get_day_runner(5).unwrap();
///     assert!(runner.is_none());
/// }
pub fn register_day_runner_default<T: DayRunner + Default + 'static>(day: u8) -> bool {
    if DAY_RUNNERS.read().unwrap().is_empty() {
        init(false);
    }

    if day == 0 {
        panic!("Trying to register runner for day 0.");
    } else if day > 25 {
        panic!("Trying to register runner for a day after the 25th.");
    }

    if DAY_RUNNERS.read().unwrap()[usize::from(day - 1)].is_some() {
        return false;
    }

    DAY_RUNNERS.write().unwrap()[usize::from(day - 1)] =
        Some(Box::new(move || Box::new(T::default())));
    true
}

/// Calls the given constructor.
///
/// This function calls the constructor from the given lock and wraps the result in a box.
///
/// # Panics
///
/// This function panics if the given [OnceLock] is uninitialized or poisoned.
fn wrap_constructor<T, U>(constructor: &OnceLock<U>) -> Box<dyn DayRunner>
where
    T: DayRunner + 'static,
    U: Fn() -> T + Send + Sync,
{
    Box::new(constructor.get().unwrap()())
}

// TODO add unit tests.

// Below are the mods for each day.
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
