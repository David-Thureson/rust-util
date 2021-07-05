#![feature(slice_partition_dedup)]
#![feature(str_split_once)]
#![macro_use]
extern crate titlecase;

pub mod convert;
pub mod date_time;
pub mod extract;
pub mod format;
pub mod file;
pub mod group;
pub mod html;
pub mod log;
pub mod parse;
pub mod stats_usize;
pub mod tree;

// type_name_of() seems to dereference automatically so it can't tell the difference between a basic value and a
// reference to that value.

#[macro_export]
macro_rules! rse {
    ($a:expr) => {
        {
            match $a {
                Ok(t) => Ok(t),
                Err(e) => Err(e.to_string()),
            }
        }
    };
}

// Same as rse!
#[macro_export]
macro_rules! result_to_string_error {
    ($a:expr) => {
        {
            match $a {
                Ok(t) => Ok(t),
                Err(e) => Err(e.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! types {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{} is {}", stringify!($x), $x.type_name_of());
            )*
        }
    };
}

#[macro_export]
macro_rules! vals {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{} = {:?}",
					stringify!($x),
					$x);
            )*
        }
    };
}

#[macro_export]
macro_rules! altvals {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{} = {:#?}",
					stringify!($x),
					$x);
            )*
        }
    };
}

#[macro_export]
macro_rules! typedvals {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{} = {:?}\n\ttype is {}",
					stringify!($x),
					$x,
					$x.type_name_of());
            )*
        }
    };
}

#[macro_export]
macro_rules! showrc {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{} = {:?}\n\ttype is {}\n\tstrong count = {}\n\tweak count = {}",
					stringify!($x),
					$x,
					$x.type_name_of(),
					Rc::strong_count(&$x),
					Rc::weak_count(&$x));
            )*
        }
    };
}

// Note that this uses $x:ty instead of $x:expr. Watch out for this if copying and pasting.
#[macro_export]
macro_rules! show_size_align {
    ( $( $x:ty ),* ) => {
        {
            $(
                println!("{:<30}: size = {:>2}; align = {:>2}; Option size = {:>2}",
					stringify!($x),
					std::mem::size_of::<$x>(),
					std::mem::align_of::<$x>(),
					std::mem::size_of::<Option<$x>>(),
				);
            )*
        }
    };
}

pub fn str_to_string_vector(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    */
}

