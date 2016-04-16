# approvals [![Build Status](https://travis-ci.org/aleksandrpak/approvals.svg)](https://travis-ci.org/aleksandrpak/approvals) [![Appveyor Build status](https://ci.appveyor.com/api/projects/status/bx0ygkhxxdm8p783/branch/master?svg=true)](https://ci.appveyor.com/project/aleksandrpak/approvals) [![Coverage Status](https://coveralls.io/repos/github/aleksandrpak/approvals/badge.svg?branch=master)](https://coveralls.io/github/aleksandrpak/approvals?branch=master) [![Clippy Linting Result](https://clippy.bashy.io/github/aleksandrpak/approvals/master/badge.svg)](https://clippy.bashy.io/github/aleksandrpak/approvals/master/log) [![MIT or Apache-2.0 licensed](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-blue.svg)](https://github.com/aleksandrpak/approvals#license) [![crates.io](http://meritbadge.herokuapp.com/approvals)](https://crates.io/crates/approvals)

Library that allows to write simple text related tests like producing JSON. 

Usage: ```approvals::approve("actual string received from test");```

## TODO
	* Make instructions to follow on failure
	* Launch diff tool on failure
	* configure folder where to keep approved files

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
