use pam::constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_OFF};
use pam::conv::Conv;
use pam::module::{PamHandle, PamHooks};
use std::collections::HashMap;
use std::ffi::CStr;
use std::time::Duration;
use pam::pam_try;

pam::pam_hooks!(PamVc);

struct PamVc;

impl PamHooks for PamVc {
    fn sm_authenticate(pamh: &mut PamHandle, args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PamResultCode::PAM_SUCCESS
    }
}
