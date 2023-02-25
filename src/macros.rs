macro_rules! invalid_format {
    ($s:expr) => {
        $crate::types::Error::InvalidFormat($s.into())
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::types::Error::InvalidFormat(format!($fmt, $($args)*).into())
    }
}

macro_rules! unexpected_eof {
    () => {
        $crate::types::Error::UnexpectedEndOfFile(None)
    };
    ($s:expr) => {
        $crate::types::Error::UnexpectedEndOfFile(Some($s.into()))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::types::Error::UnexpectedEndOfFile(Some(format!($fmt, $($args)*).into()))
    }
}

macro_rules! if_eof {
    (std, $s:expr) => {
        |e| match e {
            ref e if e.kind() == ::std::io::ErrorKind::UnexpectedEof => unexpected_eof!($s),
            e => e.into()
        }
    };
    (std, $fmt:expr, $($args:tt)*) => {
        |e| match e {
            ref e if e.kind() == ::std::io::ErrorKind::UnexpectedEof => unexpected_eof!($fmt, $($args)*),
            e => e.into()
        }
    };
    ($s:expr) => {
        |e| match e {
            ref e if e.kind() == ::std::io::ErrorKind::UnexpectedEof => unexpected_eof!($s),
            e => e.into()
        }
    };
    ($fmt:expr, $($args:tt)*) => {
        |e| match e {
            ref e if e.kind() == ::std::io::ErrorKind::UnexpectedEof => unexpected_eof!($fmt, $($args)*),
            e => e.into()
        }
    };
}

macro_rules! try_if_eof {
    (std, $e:expr, $s:expr) => {
        $e.map_err(if_eof!(std, $s))?
    };
    (std, $e:expr, $fmt:expr, $($args:tt)*) => {
        $e.map_err(if_eof!(std, $fmt, $($args)*))?
    };
    ($e:expr, $s:expr) => {
        $e.map_err(if_eof!($s))?
    };
    ($e:expr, $fmt:expr, $($args:tt)*) => {
        $e.map_err(if_eof!($fmt, $($args)*))?
    }
}
