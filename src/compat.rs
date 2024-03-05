use std::ffi::c_int;
use prost::Message;

use crate::process_context;
use crate::fixture::context::FixtureContext;
use crate::fixture::proto::{InstrContext, InstrEffects};

#[no_mangle]
pub unsafe extern "C" fn sol_compat_instr_execute_v1(
    out_ptr: *mut u8,
    out_psz: *mut u64,
    in_ptr: *mut u8,
    in_sz: u64,
) -> c_int {
    let in_slice = std::slice::from_raw_parts(in_ptr, in_sz as usize);
    let proto_context = match InstrContext::decode(in_slice) {
        Ok(context) => context,
        Err(_) => return 0,
    };
    let context: FixtureContext = match proto_context.try_into() {
        Ok(context) => context,
        Err(_) => return 0,
    };

    let effects = process_context(context);
    let proto_effects = InstrEffects::from(effects);
    let out_slice = std::slice::from_raw_parts_mut(out_ptr, (*out_psz) as usize);
    let out_vec = proto_effects.encode_to_vec();
    if out_vec.len() > out_slice.len() {
        return 0;
    }
    out_slice[..out_vec.len()].copy_from_slice(&out_vec);
    *out_psz = out_vec.len() as u64;

    1
}
