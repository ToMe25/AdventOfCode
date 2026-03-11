//! The Registry used for storing/getting the functions for running the individual parts of the solutions.
//!
//! Besides the macro [`register_runner`](crate::register_runner) it is primarily intended for internal use.
//!
//! # Examples
//!
//! Registering a runner:
//!
//! ```
//! # use std::error::Error;
//! #
//! use rust_aoc_2025::register_runner;
//! use rust_aoc_2025::dayrunner::{DayRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext};
//!
//! /// A simple example runner.
//! struct ExampleRunner();
//!
//! impl DayRunner for ExampleRunner {
//!     fn init(_ctx: &RunContext) -> Result<Self, Box<dyn Error>>
//!     where
//!         Self: Sized,
//!     {
//!         Ok(ExampleRunner())
//!     }
//! }
//!
//! // Specify which day this runner implements.
//! impl DayRunnerDate for ExampleRunner {
//!     const DAY: u8 = 3;
//! }
//!
//! // Implement a solution for part one.
//! impl Part1Runner for ExampleRunner {
//!     fn part1(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>> {
//!         assert_eq!(Self::DAY, ctx.day);
//!         Ok(format!("Day {} part 1", Self::DAY).to_owned())
//!     }
//! }
//!
//! // Register the runner.
//! // It can now be executed using the runner system.
//! register_runner!(ExampleRunner);
//!```

use std::array;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{LazyLock, OnceLock};

use super::{BothPartsRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext};

/// The number of days that exist for the current [Advent of Code](https://adventofcode.com/).
pub const MAX_DAY: u8 = 12;

static REGISTRY: LazyLock<[OnceLock<RunnerData>; MAX_DAY as usize + 1]> =
    LazyLock::new(|| array::from_fn(|_| OnceLock::new()));

/// Registers a [`DayRunner`](crate::dayrunner::DayRunner) to be used with this registry.
///
/// Can only handle registering runners that implement at least one of the two parts.  
/// TODO find a way to handle the none case.
///
/// # Examples
///
/// See the [module level docs](crate::dayrunner::registry).
#[macro_export]
macro_rules! register_runner {
    ($t:ty) => {{
        use std::marker::PhantomData;
        #[allow(unused_imports)]
        use $crate::dayrunner::registry::DayRunnerBothPartsTrait;
        #[allow(unused_imports)]
        use $crate::dayrunner::registry::DayRunnerPart1Trait;
        #[allow(unused_imports)]
        use $crate::dayrunner::registry::DayRunnerPart2Trait;
        (&PhantomData::<$t>::default()).register()
    }};
}

/// A helper enum to store which days have implementaions for which parts.
pub(super) enum RunnerType {
    // An implementation that has neither a part 1 implementation nor a part 2 implementation.
    //#[allow(unused)]
    //None(Box<dyn DayRunner>),
    /// A day runner that has a part 1 implementation, but no part 2 implementation.
    #[allow(unused)]
    Part1(Box<dyn Part1Runner>),
    /// A day runner that has a part 2 implementation, but no part 1 implementation.
    #[allow(unused)]
    Part2(Box<dyn Part2Runner>),
    /// An implementation that can solve both parts of a day.
    #[allow(unused)]
    Both(Box<dyn BothPartsRunner>),
}

impl fmt::Debug for RunnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RunnerType::Part1(_) => "Part1",
                RunnerType::Part2(_) => "Part2",
                RunnerType::Both(_) => "Both",
            }
        )
    }
}

impl RunnerType {
    /// Attempt to run part 1 of a solution.
    ///
    /// Returns the result of the day solver if it implements a part one solution, or [`None`].
    pub fn part1(&self, ctx: &RunContext) -> Option<Result<String, Box<dyn Error>>> {
        match self {
            Self::Part1(runner) => Some(runner.part1(ctx)),
            Self::Both(runner) => Some(runner.part1(ctx)),
            _ => None,
        }
    }

    /// Attempt to run part 2 of a solution.
    ///
    /// Returns the result of the day solver if it implements a part two solution, or [`None`].
    pub fn part2(&self, ctx: &RunContext) -> Option<Result<String, Box<dyn Error>>> {
        match self {
            Self::Part2(runner) => Some(runner.part2(ctx)),
            Self::Both(runner) => Some(runner.part2(ctx)),
            _ => None,
        }
    }
}

/// A helper struct used to instantiate [`RunnerType`] instances.
struct RunnerData {
    /// The initialization function used to create a new [`RunnerType`] instance.
    init_fn: Box<dyn Fn(&RunContext) -> Result<RunnerType, Box<dyn Error>> + Send + Sync>,
}

impl RunnerData {
    /// Initializes a new runner for the day represented by this RunnerData.
    ///
    /// # Errors
    ///
    /// This function will return whatever error the runner itself returns.
    #[must_use = "does not update this object"]
    fn init(&self, ctx: &RunContext) -> Result<RunnerType, Box<dyn Error>> {
        (self.init_fn)(ctx)
    }
}

macro_rules! register_runner_wrap_init {
    ($part_trait:path,$runner_type:path,$register_fn:ident,$wrap_fn:ident) => {
        /// Wraps a [`DayRunner`] init function in another function to return a [`RunnerType`] instead of a runner.
        ///
        /// This function does not run the init function.
        fn $wrap_fn<T: $part_trait + 'static>(
            fun: Box<dyn Fn(&RunContext) -> Result<T, Box<dyn Error>> + Send + Sync>,
        ) -> Box<dyn Fn(&RunContext) -> Result<RunnerType, Box<dyn Error>> + Send + Sync> {
            Box::new(move |ctx| fun(ctx).map(|runner| $runner_type(Box::new(runner))))
        }

        /// Registers a runner type with the parts indicated by the function name to the runner registry.
        ///
        /// If a runner is already registered for the same day, even if not for the same part, the runner will not be registered.
        /// A log message will be printed in this case.
        pub fn $register_fn<T: $part_trait + DayRunnerDate + 'static>() {
            assert!(
                T::DAY <= MAX_DAY,
                "trying to register runner for invalid day {}",
                T::DAY
            );
            if REGISTRY[Into::<usize>::into(T::DAY)]
                .set(RunnerData {
                    init_fn: $wrap_fn(Box::new(T::init)),
                })
                .is_err()
            {
                eprintln!(
                    "Not registering day {} runner because one is already registerd.",
                    T::DAY
                );
            }
        }
    };
}

/*register_runner_wrap_init!(
    DayRunner,
    RunnerType::None,
    register_none_runner,
    wrap_none_init
);*/
register_runner_wrap_init!(
    Part1Runner,
    RunnerType::Part1,
    register_part_1_runner,
    wrap_part_1_init
);
register_runner_wrap_init!(
    Part2Runner,
    RunnerType::Part2,
    register_part_2_runner,
    wrap_part_2_init
);
register_runner_wrap_init!(
    BothPartsRunner,
    RunnerType::Both,
    register_both_runner,
    wrap_both_init
);

/// Creates a new runner for the day specified by the given [`RunContext`].
///
/// This function returns [`None`] if no runner was registered for the given day.  
/// It does not, however, check whether the registered runner implements a solution for the part(s) specified by the [`RunContext`].
///
/// When trying to get a runner for a day after the last day, it will return [`None`].
///
/// # Errors
///
/// This function returns whatever error the init function of the runner returns, if it fails.
pub(super) fn get_runner_for_day(ctx: &RunContext) -> Option<Result<RunnerType, Box<dyn Error>>> {
    if ctx.day > MAX_DAY {
        return None;
    }
    REGISTRY[Into::<usize>::into(ctx.day)]
        .get()
        .map(|rd| rd.init(ctx))
}

macro_rules! register_part_impl {
    ($part_trait:path,$trait_name:ident,$register_fn:ident,$($ref:tt)?) => {
        /// A helper trait for [`Autoref Specialization`](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md)
        #[allow(unused)]
        #[doc(hidden)]
        pub trait $trait_name {
            /// Registers the given runner type to the global runner registry.
            fn register(&self);
        }

        impl<T: $part_trait + DayRunnerDate + 'static> $trait_name for $($ref)? PhantomData<T> {
            fn register(&self) {
                $register_fn::<T>();
            }
        }
    };
}

register_part_impl!(
    BothPartsRunner,
    DayRunnerBothPartsTrait,
    register_both_runner,
);
register_part_impl!(Part1Runner, DayRunnerPart1Trait, register_part_1_runner, &);
register_part_impl!(Part2Runner, DayRunnerPart2Trait, register_part_2_runner, &);

/*#[allow(unused)]
trait DayRunnerNoPartTrait {
    fn register(&self);
}

impl<T: DayRunner + DayRunnerDate + 'static> DayRunnerNoPartTrait for &&PhantomData<T> {
    fn register(&self) {
        register_none_runner::<T>();
    }
}*/

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::super::{DayRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext};
    use super::get_runner_for_day;

    /// Generates a runner struct implementing [`DayRunner`] and [`DayRunnerDate`].
    #[macro_export]
    macro_rules! base_test_runner {
        ($runner_name:ident,$day:expr) => {
            struct $runner_name();

            impl DayRunner for $runner_name {
                fn init(_: &RunContext) -> Result<Self, Box<dyn Error>>
                where
                    Self: Sized,
                {
                    Ok($runner_name())
                }
            }

            impl DayRunnerDate for $runner_name {
                const DAY: u8 = $day;
            }
        };
    }

    /// Generates a part 1 or part 2 implementation for the given runner struct.
    #[macro_export]
    macro_rules! test_runner_part {
        ($runner_name:ident,1) => {
            $crate::test_runner_part!($runner_name, Part1Runner, part1, 1);
        };
        ($runner_name:ident,2) => {
            $crate::test_runner_part!($runner_name, Part2Runner, part2, 2);
        };
        ($runner_name:ident,$runner_part_trait:ident,$part_fn:ident,$part_nr:literal) => {
            impl $runner_part_trait for $runner_name {
                fn $part_fn(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>> {
                    assert_eq!(
                        Self::DAY,
                        ctx.day,
                        "This runner was designed for day {}, but run for day {}.",
                        Self::DAY,
                        ctx.day
                    );
                    assert!(
                        ctx.$part_fn,
                        concat!(
                            "This runner was desinged for part ",
                            stringify!($part_nr),
                            " but run without it enabled."
                        )
                    );
                    Ok(
                        format!(concat!("Day {} part ", stringify!($part_nr)), Self::DAY)
                            .to_owned(),
                    )
                }
            }
        };
    }

    base_test_runner!(TestRunnerPart1, 7);
    test_runner_part!(TestRunnerPart1, 1);

    #[test]
    fn register_part_1_runner() {
        register_runner!(TestRunnerPart1);
    }

    #[test]
    fn get_runner_for_missing_day() {
        let runner = get_runner_for_day(&RunContext::create_default_for_day(5));
        assert!(runner.is_none(), "Runner {:?} wasn't None.", runner);
    }

    #[test]
    fn get_runner() {
        register_runner!(TestRunnerPart1);
        let runner = get_runner_for_day(&RunContext::create_default_for_day(7));
        assert!(runner.is_some(), "There wasn't a runner for day 7.");
        assert!(
            runner.as_ref().unwrap().is_ok(),
            "Initializing a runner for day 7 failed with error {:?}",
            runner.as_ref().unwrap().as_ref().unwrap_err()
        );
    }

    #[test]
    fn run_part1() {
        register_runner!(TestRunnerPart1);
        let ctx = RunContext::create_default_for_day(7);
        let runner = get_runner_for_day(&ctx);
        assert!(
            runner.is_some(),
            "There wasn't a runner for day {}.",
            ctx.day
        );
        assert!(
            runner.as_ref().unwrap().is_ok(),
            "Initializing a runner for day {} failed with error {:?}",
            ctx.day,
            runner.as_ref().unwrap().as_ref().unwrap_err()
        );
        let part_1_result = runner.as_ref().unwrap().as_ref().unwrap().part1(&ctx);
        assert!(
            part_1_result.is_some(),
            "Part 1 wasn't detected to be implemented."
        );
        assert!(part_1_result.as_ref().unwrap().is_ok(), "Part 1 failed.");
        assert_eq!(
            part_1_result.as_ref().unwrap().as_ref().unwrap(),
            &format!("Day {} part 1", ctx.day),
            "Runner part 1 result mismatch"
        );
        let part_2_result = runner.as_ref().unwrap().as_ref().unwrap().part2(&ctx);
        assert!(
            part_2_result.is_none(),
            "Part 2 was detected to be implemented."
        );
    }

    #[test]
    fn get_invalid_day() {
        let runner = get_runner_for_day(&RunContext::create_default_for_day(26));
        assert!(runner.is_none(), "Runner {:?} wasn't None.", runner);
    }

    base_test_runner!(TestRunnerPart2, 9);
    test_runner_part!(TestRunnerPart2, 2);

    #[test]
    fn run_part2() {
        register_runner!(TestRunnerPart2);
        let ctx = RunContext::create_default_for_day(9);
        let runner = get_runner_for_day(&ctx);
        assert!(
            runner.is_some(),
            "There wasn't a runner for day {}.",
            ctx.day
        );
        assert!(
            runner.as_ref().unwrap().is_ok(),
            "Initializing a runner for day {} failed with error {:?}",
            ctx.day,
            runner.as_ref().unwrap().as_ref().unwrap_err()
        );
        let part_1_result = runner.as_ref().unwrap().as_ref().unwrap().part1(&ctx);
        assert!(
            part_1_result.is_none(),
            "Part 1 was detected to be implemented."
        );
        let part_2_result = runner.as_ref().unwrap().as_ref().unwrap().part2(&ctx);
        assert!(
            part_2_result.is_some(),
            "Part 2 wasn't detected to be implemented."
        );
        assert!(part_2_result.as_ref().unwrap().is_ok(), "Part 2 failed.");
        assert_eq!(
            part_2_result.as_ref().unwrap().as_ref().unwrap(),
            &format!("Day {} part 2", ctx.day),
            "Runner part 2 result mismatch"
        );
    }

    base_test_runner!(TestRunnerBothParts, 11);
    test_runner_part!(TestRunnerBothParts, 1);
    test_runner_part!(TestRunnerBothParts, 2);

    #[test]
    fn run_both_parts() {
        register_runner!(TestRunnerBothParts);
        let ctx = RunContext::create_default_for_day(11);
        let runner = get_runner_for_day(&ctx);
        assert!(
            runner.is_some(),
            "There wasn't a runner for day {}.",
            ctx.day
        );
        assert!(
            runner.as_ref().unwrap().is_ok(),
            "Initializing a runner for day {} failed with error {:?}",
            ctx.day,
            runner.as_ref().unwrap().as_ref().unwrap_err()
        );
        let part_1_result = runner.as_ref().unwrap().as_ref().unwrap().part1(&ctx);
        assert!(
            part_1_result.is_some(),
            "Part 1 wasn't detected to be implemented."
        );
        assert!(part_1_result.as_ref().unwrap().is_ok(), "Part 1 failed.");
        assert_eq!(
            part_1_result.as_ref().unwrap().as_ref().unwrap(),
            &format!("Day {} part 1", ctx.day),
            "Runner part 1 result mismatch"
        );
        let part_2_result = runner.as_ref().unwrap().as_ref().unwrap().part2(&ctx);
        assert!(
            part_2_result.is_some(),
            "Part 2 wasn't detected to be implemented."
        );
        assert!(part_2_result.as_ref().unwrap().is_ok(), "Part 2 failed.");
        assert_eq!(
            part_2_result.as_ref().unwrap().as_ref().unwrap(),
            &format!("Day {} part 2", ctx.day),
            "Runner part 2 result mismatch"
        );
    }
}
