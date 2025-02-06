// Copyright 2022 Google LLC
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

use crate::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};
use num_traits::float::Float;
use std::fmt::Debug;

/// Matches a floating point value which is Infinite.
pub fn is_infinite() -> IsInfiniteMatcher {
    IsInfiniteMatcher
}

#[derive(MatcherBase)]
pub struct IsInfiniteMatcher;

impl<T: Float + Debug + Copy> Matcher<T> for IsInfiniteMatcher {
    fn matches(&self, actual: T) -> MatcherResult {
        actual.is_infinite().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        if matcher_result.into() { "is Infinite" } else { "isn't Infinite" }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Result;

    #[test]
    fn matches_f32_pos_infinity() -> Result<()> {
        verify_that!(f32::INFINITY, is_infinite())
    }

    #[test]
    fn matches_f32_neg_infinity() -> Result<()> {
        verify_that!(f32::NEG_INFINITY, is_infinite())
    }

    #[test]
    fn does_not_match_f32_number() -> Result<()> {
        verify_that!(0.0f32, not(is_infinite()))
    }

    #[test]
    fn matches_f64_pos_infinity() -> Result<()> {
        verify_that!(f64::INFINITY, is_infinite())
    }

    #[test]
    fn matches_f64_neg_infinity() -> Result<()> {
        verify_that!(f64::NEG_INFINITY, is_infinite())
    }

    #[test]
    fn does_not_match_f64_number() -> Result<()> {
        verify_that!(0.0f64, not(is_infinite()))
    }
}
