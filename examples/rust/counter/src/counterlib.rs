// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use counter_core::{
    CounterState, InitializeCounterCommit, SubmitCounterCommit, SubmitCounterParams,
    SubmitCounterResult,
};
use risc0_zkvm_host::{Prover, Receipt, Result};
use risc0_zkvm_serde::{from_slice, to_vec};
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn init_stuff() {
    env_logger::init();

    log::trace!("init_stuff() trace level message");
    log::debug!("init_stuff() debug level message");
    log::info!("init_stuff() info level message");
    log::warn!("init_stuff() warn level message");
    log::error!("init_stuff() error level message");
}

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello {}!", str_name);
}

#[no_mangle]
pub extern "C" fn create_counter_station() -> *mut CounterStation {
    let counter_station_state = CounterState { count: 0 };
    let counter_station = CounterStation::new(counter_station_state);
    let boxed = Box::new(counter_station);
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "C" fn counter_station_init(counter_station: *mut CounterStation) -> *mut InitMessage {
    let init_msg = unsafe { (*counter_station).init().unwrap() };
    let boxed = Box::new(init_msg);
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "C" fn counter_station_submit(
    counter_station: *mut CounterStation,
) -> *mut SubmitCounterMessage {
    let msg = unsafe { (*counter_station).submit().unwrap() };
    let boxed = Box::new(msg);
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "C" fn verify_and_get_commit_init(init_msg: *mut InitMessage) -> *const libc::c_char {
    let init_state = unsafe { (*init_msg).verify_and_get_commit().unwrap() };
    CString::new(serde_json::to_string(&init_state).unwrap())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn verify_and_get_commit(msg: *mut SubmitCounterMessage) -> *const libc::c_char {
    let commit = unsafe { (*msg).verify_and_get_commit().unwrap() };
    CString::new(serde_json::to_string(&commit).unwrap())
        .unwrap()
        .into_raw()
}

#[repr(C)]
pub struct InitMessage {
    pub receipt: Receipt,
}

impl InitMessage {
    pub fn get_state(&self) -> Result<InitializeCounterCommit> {
        let msg = self.receipt.get_journal_vec()?;
        Ok(from_slice(msg.as_slice()).unwrap())
    }

    pub fn verify_and_get_commit(&self) -> Result<InitializeCounterCommit> {
        self.receipt.verify("circuit/init")?;
        self.get_state()
    }
}

#[repr(C)]
pub struct SubmitCounterMessage {
    pub receipt: Receipt,
}

impl SubmitCounterMessage {
    pub fn get_commit(&self) -> Result<SubmitCounterCommit> {
        let msg = self.receipt.get_journal_vec()?;
        Ok(from_slice(msg.as_slice()).unwrap())
    }

    pub fn verify_and_get_commit(&self) -> Result<SubmitCounterCommit> {
        self.receipt.verify("circuit/submit")?;
        self.get_commit()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CounterStation {
    pub state: CounterState,
}

impl CounterStation {
    pub fn new(state: CounterState) -> Self {
        CounterStation { state }
    }

    pub fn init(&self) -> Result<InitMessage> {
        log::info!("init");
        let mut prover = Prover::new("circuit/init")?;
        let vec = to_vec(&self.state).unwrap();
        prover.add_input(vec.as_slice())?;
        let receipt = prover.run()?;
        Ok(InitMessage { receipt })
    }

    pub fn submit(&mut self) -> Result<SubmitCounterMessage> {
        log::info!("submit, increase the counter by 1");
        let params = SubmitCounterParams::new(self.state.clone());
        let mut prover = Prover::new("circuit/submit")?;
        let vec = to_vec(&params).unwrap();
        prover.add_input(vec.as_slice())?;
        let receipt = prover.run()?;
        let vec = prover.get_output_vec()?;
        let result = from_slice::<SubmitCounterResult>(vec.as_slice()).unwrap();
        self.state = result.state.clone();
        Ok(SubmitCounterMessage { receipt })
    }
}
