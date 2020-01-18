use crate::defines::defines::*;

pub fn cmd(_user: &mut ServerConnection) ->
Result<(), ClientError> {
    _user.connect_mode = FTPModes::Active;
    return Err(ClientError::Regular(ErrorKind::NoWait));
}
