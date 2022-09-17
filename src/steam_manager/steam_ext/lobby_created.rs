/*use std::ffi::c_void;
use steamworks::Callback;

pub type uint64 = ::std::os::raw::c_ulonglong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]/**/
pub enum EResult {
    k_EResultOK,
    k_EResultFail,
    k_EResultTimeout,
    k_EResultLimitExceeded,
    k_EResultAccessDenied,
    k_EResultNoConnection
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LobbyCreated_t {
    m_eResult: EResult,
    m_ulSteamIDLobby: uint64
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LobbyCreated {

}

unsafe impl Callback for LobbyCreated {
    const ID: i32 = 513;
    const SIZE: i32 = std::mem::size_of::<LobbyCreated_t>() as i32;

    unsafe fn from_raw(raw: *mut c_void) -> Self {
        let val = &mut *(raw as *mut LobbyCreated_t);
        todo!()
    }
}*/