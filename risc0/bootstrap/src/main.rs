// Copyright 2023 RISC Zero, Inc.
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

use risc0_zkvm::Loader;

fn main() {
    let loader = Loader::new();
    let control_id = loader.compute_control_id();
    let contents = format!(
        include_str!("control_id.rs"),
        control_id.table[0],
        control_id.table[1],
        control_id.table[2],
        control_id.table[3],
        control_id.table[4],
        control_id.table[5],
        control_id.table[6],
        control_id.table[7],
        control_id.table[8],
        control_id.table[9],
        control_id.table[10],
        control_id.table[11],
        control_id.table[12],
    );
    println!("{contents}");
    std::fs::write("risc0/zkvm/src/control_id.rs", contents).unwrap();
}
