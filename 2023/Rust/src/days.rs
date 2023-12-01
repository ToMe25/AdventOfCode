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
//! struct SomeRunner {}
//!
//! impl SomeRunner {
//!     pub fn new() -> SomeRunner {
//!         SomeRunner {}
//!     }
//! }
//!
//! impl DayRunner for SomeRunner {}
//!
//! fn main() {
//!     days::init();
//!     let _ = days::register_day_runner(19, SomeRunner::new);
//!     let mut runner = days::get_day_runner(19).unwrap();
//!     assert!(runner.is_some());
//!     runner = days::get_day_runner(5).unwrap();
//!     assert!(runner.is_none());
//! }
//! ```

use std::error::Error;
use std::sync::{OnceLock, RwLock};

use self::day1::Day1Runner;

static DAY_RUNNERS: RwLock<Vec<Option<Box<dyn Fn() -> Box<dyn DayRunner> + Send + Sync>>>> =
    RwLock::new(Vec::new());

/// Initializes the internal runners list.
///
/// Initializes the interal data structure storing the constructors for each days runner.  
/// Has to be called before the first [get_day_runner] call.  
/// Currently also registers all the already implemented days.
///
/// Will automatically be called by the first [register_day_runner] call.  
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
/// days::init();
/// ```
pub fn init() {
    if DAY_RUNNERS.read().unwrap().is_empty() {
        DAY_RUNNERS.write().unwrap().extend((0..25).map(|_| None));
        register_day_runner(1, Day1Runner::new);
    }
}

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
/// }
///
/// fn main() {
///     let mut runner = ExampleRunner {common: 0};
///     let _ = runner.init();
///     runner.part1();
///     runner.part2();
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
    /// It is allowed to return None, first for not yet implemented parts,
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
    /// It is allowed to return None, first for not yet implemented parts,
    /// and second for the rare cases in which the output has to be directly printed to allow easy reading.
    ///
    /// # Errors
    ///
    /// Depending on the implementation, this method is allowed to return arbitrary types of errors.
    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        Ok(None)
    }
}

/// Get a runner for a specific day.
///
/// This function creates a new runner for the given day, if a constructor was registered for that day.  
/// Constructors can be registered using [register_day_runner].  
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
/// Note that [register_day_runner] automatically runs [init].
///
/// # Examples
///
/// ```
/// # use rust_aoc_2023::days;
/// # use rust_aoc_2023::days::DayRunner;
/// #
/// # struct SomeRunner {}
/// #
/// # impl SomeRunner {
/// #     pub fn new() -> SomeRunner {
/// #         SomeRunner {}
/// #     }
/// # }
/// #
/// # impl DayRunner for SomeRunner {}
/// #
/// # fn main() {
///     days::init();
///     let _ = days::register_day_runner(19, SomeRunner::new);
///     let mut runner = days::get_day_runner(19).unwrap();
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

/// Register a runner for a specific day.
///
/// This function registers the constructor to be used to create a runner for the given day.  
/// To get a runner use [get_day_runner].
///
/// # Panics
///
/// This function panics if given a day that is either 0 or above 25.  
/// And if the internal [RwLock] is poisoned.
///
/// # Examples
///
/// ```
/// # use rust_aoc_2023::days;
/// # use rust_aoc_2023::days::DayRunner;
/// #
/// # struct SomeRunner {}
/// #
/// # impl SomeRunner {
/// #     pub fn new() -> SomeRunner {
/// #         SomeRunner {}
/// #     }
/// # }
/// #
/// # impl DayRunner for SomeRunner {}
/// #
/// # fn main() {
///     days::init();
///     let _ = days::register_day_runner(6, SomeRunner::new);
///     let mut runner = days::get_day_runner(4).unwrap();
///     assert!(runner.is_none());
///     runner = days::get_day_runner(6).unwrap();
///     assert!(runner.is_some());
/// # }
/// ```
pub fn register_day_runner<T, U>(day: u8, constructor: U) -> bool
where
    T: DayRunner + 'static,
    U: Fn() -> T + Send + Sync + 'static,
{
    if DAY_RUNNERS.read().unwrap().is_empty() {
        init();
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
mod day1;
