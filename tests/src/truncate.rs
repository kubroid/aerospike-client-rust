// Copyright 2015-2018 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use aerospike::WritePolicy;

use env_logger;
use common;

#[test]
fn truncate() {
    let _ = env_logger::init();

    let client = common::client();
    let namespace: &str = common::namespace();
    let set_name = &common::rand_str(10);
    let wpolicy = WritePolicy::default();

    let result = client.truncate(&wpolicy, namespace, set_name, 0);
    assert!(result.is_ok());
}
