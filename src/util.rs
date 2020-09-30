macro_rules! tail {
  ($head:tt $($tail:tt)*) => ($($tail)*)
}

macro_rules! fold {
  ($op:tt $($el:expr),*) => (tail!($($op $el)*))
}
