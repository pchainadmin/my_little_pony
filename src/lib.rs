/*
 Copyright (c) 2022 ParallelChain Lab

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn alloc(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    let a = 45;
    return ptr;
}

extern "C" {
    pub(crate) fn set(_: *const u8, _: u32, _: *const u8, _: u32);
    pub(crate) fn get(_: *const u8, _: u32, _: *const u32) -> i64;
    pub(crate) fn get_transaction_from_address(_: *const u32) -> u32;
    pub(crate) fn get_transaction_to_address(_: *const u32) -> u32;
    pub(crate) fn get_transaction_value(_: *const u32) -> u32;
    pub(crate) fn get_transaction_nonce(_: *const u32) -> u32;
    pub(crate) fn get_transaction_hash(_: *const u32) -> u32;
    pub(crate) fn get_transaction_data(_: *const u32) -> u32;
    pub(crate) fn get_blockchain_height(_: *const u32) -> u32;
    pub(crate) fn get_blockchain_prev_hash(_: *const u32) -> u32;
    pub(crate) fn get_blockchain_timestamp(_: *const u32) -> u32;
    pub(crate) fn get_blockchain_random_bytes(_: *const u32) -> u32;
    pub(crate) fn emit(_: *const u8, _: u32);
    pub(crate) fn return_value(_: *const u8, _: u32);
    pub(crate) fn call_action(_: *const u8, _: *const u8, _ :u32, _: *const u8, _: *const u32) -> u32;
    pub(crate) fn call_view(_: *const u8, _: *const u8, _: u32, _: *const u32) -> u32;
    pub(crate) fn pay(_: *const u8, _ : *const u8) -> u64;
    pub(crate) fn random() -> u64;
    pub(crate) fn sha256(_: *const u8, _: u32, _: *const u32) -> u32;
    pub(crate) fn keccak256(_: *const u8, _: u32, _: *const u32) -> u32;
    pub(crate) fn keccak512(_: *const u8, _: u32, _: *const u32) -> u32;
    pub(crate) fn ripemd160(_: *const u8, _: u32, _: *const u32) -> u32;
    pub(crate) fn blake2b(_: *const u8, _: u32, _: u32, _: *const u32) -> u32;
    pub(crate) fn verify_signature(_: *const u8, _: u32, _: *const u8, _: *const u8) -> i32;
}

#[no_mangle]
pub extern "C" fn actions() {}
