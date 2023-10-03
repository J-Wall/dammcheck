Dammcheck
=========

Implementation of the [Damm Algorithm](https://en.wikipedia.org/wiki/Damm_algorithm)
in various bases/alphabets.

Usage
-----

```
import dammcheck

# List available built-in alphabets
dammcheck.builtin_alphabets()

# Calculate the Damm check digit using a built-in alphabet
dammcheck.damm("ABC", alphabet="base32crockford")

# Or a custom alphabet
dammcheck.damm("TATA", custom_alphabet="ACGT")
```
