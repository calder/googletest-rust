// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Utilities to facilitate writing matchers.
//!
//! Tests normally do not need to import anything from this module. Some of
//! these facilities could be useful to downstream users writing custom
//! matchers.

mod auto_eq;
pub(crate) mod count_elements;
pub(crate) mod edit_distance;
pub(crate) mod match_matrix;
pub(crate) mod summarize_diff;
pub(crate) mod zipped_iterator;

pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::auto_eq::internal::{ExpectedKind, Wrapper};
    pub use crate::__auto_eq as auto_eq;
}
