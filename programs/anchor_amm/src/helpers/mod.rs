#[macro_export]
macro_rules! assert_non_zero {
    ($array:expr) => {
        if $array.contains(&0u64) {
            return err!(AmmError::ZeroBalance)
        }
    };
}

#[macro_export]
macro_rules! assert_has_authority {
    ($x:expr) => {
        match $x.config.authority {
            Some(authority) => {
                require_keys_eq!(authority, $x.user.key)(, AmmError::InvalidAuthority)
            },
            None => return err!(AmmError::Unauthorized)
        }
    };
}

#[macro_export]
macro_rules! assert_not_locked {
    ($x:expr) => {
        if ($x.config.locked == true) {
            return err!(AmmError::PoolLocked)
        }
    };
}