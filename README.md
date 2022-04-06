# TriPeaks Solitaire Solver

This is a TriPeaks Solitaire solver written in Rust, based on [Courtney Pitcher (@IgniparousTempest)'s original](https://github.com/IgniparousTempest/javascript-tri-peaks-solitaire-solver).

It uses mostly the same algorithm as the original, but modified to be slightly more efficient (using fewer allocations).

I made this because the JavaScript version is very slow for difficult boards, and I wanted to see how Rust would perform. Additionally, the JavaScript solver occasionally seems to combine stock moves; which can make the solutions it produces difficult to understand.
