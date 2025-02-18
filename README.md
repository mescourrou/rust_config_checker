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
3) Add the rules to the fields you want checked.
4) For fields which need conversion (such as `String`), use `#[convert(my_convert_function)]`.

Rules:
- `ge`, `gt`, `le`, `lt`, `ne`, `eq`: comparison operators. Can be used with one argument (comparing the field), or with two (usefull to use in the `if` rule)
- `and`, `or`: logic operators, can take as many arguments as needed.
- `if`: only apply a rule on a condition. Usage: `if(cond, if_true [, if_false])`.
- `inside`: can take check if the field is inside the given list (arguments). e.g.: `inside("A", "D", "E")`.
- `is_enum`: check if the field is a value of the enum. E.g. `is_enum(MyEnum::MyVariant)`. For enum with types, don't forget `(_)`: `is_enum(MyEnum::MyVariant(_))`

## Todo:
- [ ] Rules for vectors (for_all, any, all, ...)
- [ ] Give more precise information when the rule is not fulfilled