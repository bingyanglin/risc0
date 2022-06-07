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

#![cfg_attr(not(test), no_std)]

use risc0_zkvm_core::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(C)]
pub struct CounterState {
    pub count: u32,
}

impl CounterState {
    pub fn increase(&mut self) {
        self.count += 1;
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(C)]
pub struct InitializeCounterCommit {
    pub state: Digest,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(C)]
pub struct SubmitCounterCommit {
    pub old_state: Digest,
    pub new_state: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(C)]
pub struct SubmitCounterParams {
    pub state: CounterState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(C)]
pub struct SubmitCounterResult {
    pub state: CounterState,
}

impl SubmitCounterParams {
    pub fn new(state: CounterState) -> Self {
        SubmitCounterParams { state: state }
    }

    pub fn process(&self) -> SubmitCounterResult {
        let mut state = self.state.clone();
        SubmitCounterResult { state: state }
    }
}
