use self::error::RemoveUserError;

pub mod error;

#[inline]
pub fn command() -> Result<(), RemoveUserError> {
    Ok(())
}
