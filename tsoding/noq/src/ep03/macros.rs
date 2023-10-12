
#[macro_export]
macro_rules! token_knid_enum {
    ($($kinds: ident),* $(,)?) => {
        #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
        pub enum TokenKind {
          $($kinds),*
        }
        pub const TOKEN_KIND_ITEMS: [TokenKind; [$(TokenKind::$kinds),*].len()] = [$(TokenKind::$kinds),*];
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

#[macro_export]
macro_rules! fun_args {
  () => { vec![] };
  ($name: ident) => { vec![expr!($name)] };
  ($name: ident, $($rest: tt)*) => {
      {
          let mut t = vec![expr!($name)];
          t.append(&mut fun_args!($($rest)*));
          t
      }
  };
  ($name: ident($($args: tt)*)) => {
      vec![expr!($name($($args)*))]
  };
  ($name: ident($($args: tt)*), $($rest: tt)*) => {
      {
          let mut t = vec![expr!($name($($args)*))];
          t.append(&mut fun_args!($($rest)*));
          t
      }
  }
}

#[macro_export]
macro_rules! expr {
  ($name: ident) => {
      Sym(stringify!($name).to_string())
  };
  ($name: ident($($args: tt)*)) => {
      Fun(stringify!($name).to_string(), fun_args!($($args)*))
  };
}