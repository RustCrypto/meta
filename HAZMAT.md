# About "hazmat" crates ⚠️

![hazmat](https://img.shields.io/badge/crypto-hazmat%E2%9A%A0-red)

Crates with a "hazmat" badge provide *low-level cryptographic functionality*
which is difficult to use correctly and if used incorrectly will
*potentially fail catastrophically*.

These crates are typically building blocks for higher-level constructions, and
should come with a recommendation of a safer, higher-level construction to use.

When in doubt, avoid using these crates directly unless you really need them
(ideally as building blocks for higher-level constructions).
