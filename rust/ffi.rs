// Copyright 2017 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libc::c_int;

extern "C" {
    pub fn add(a: c_int, b: c_int) -> c_int;
    pub fn subtract(a: c_int, b: c_int) -> c_int;
    pub fn multiply(a: c_int, b: c_int) -> c_int;
    pub fn divide(a: c_int, b: c_int) -> f64;
}
