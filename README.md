# Config Checker

Rust tool to add logic rules to configuration structs. The struct has then the `ConfigurationCheckable` trait,
which implement `.check()`. This function returns true if all rules are respected. Otherwise it returns false
and prints which field is not compliant.

## Example
An example can be found in the `example` directory.

## Use
1) Add the `Check` derive macro to all struct and enums you want checked.
2) If an attribute is a sub-configuration which need to be checked, use `#[check]` attribute. There is no
   auto-discover for structs that derive `Check`.
3) If there are specific rules for this struct, implement `Check` trait (`do_check` method).