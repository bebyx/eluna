# eluna

A Rust translation of [Minkukel's algorithm](https://minkukel.com/en/various/calculating-moon-phase/), which claims to provide **lunar data for a period of 1900-2100**.

Input is a timestamp (`i64`), whether positive or negative. It's compatible with 
[chrono `timestamp()`](https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.timestamp) function.

Outputs are raw (moon second for the given time), fraction of the moon, a moon day, and a phase (numeric or English).

Results are rather precise, when close to 2000, but may be quite approximate for the distant dates.
