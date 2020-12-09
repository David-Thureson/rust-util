#![feature(slice_partition_dedup)]
#![feature(str_split_once)]
extern crate titlecase;

pub mod format;
pub mod group;
pub mod html;
pub mod log;
pub mod parse;

// type_name_of() seems to dereference automatically so it can't tell the difference between a basic value and a
// reference to that value.

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

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    */
}

