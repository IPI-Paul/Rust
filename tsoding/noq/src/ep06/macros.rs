#[macro_export]
macro_rules! loc_here {
    () => {
        Loc {
            file_path: Some(file!().to_string()),
            row: line!() as usize,
            col: column!() as usize
        }
    }
}

// Not needed in Example_07.
#[macro_export]
macro_rules! sym {
  ($name: ident) => {
      Sym(stringify!($name).to_string())
  };
}

// Not needed in Example_07.
#[macro_export]
macro_rules! fun {
  ($name: ident) => {
      Fun(stringify!($name).to_string(), vec![])
  };
  // args is a repeating group of tokens with a comma sparator. Like Regex * is repeating type.
  ($name: ident, $($args:expr),*) => {
      Fun(stringify!($name).to_string(), vec![$($args),*])
  };
}
