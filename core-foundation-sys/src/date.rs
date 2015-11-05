// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub type CFTimeInterval = f64;
pub type CFAbsoluteTime = CFTimeInterval;

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    pub fn CFAbsoluteTimeGetCurrent() -> CFAbsoluteTime;
}