# Info
- IEEE 745 (a standard for floating point precision)
- CPU Microcode will change the results (I think)

Untested:
- CPUs of same generation, a Ryzen 5500 and 5800X might return the same fingerprint
- If Microcode revisions change the results

Tested with:
- Ryzen 9 Zen 5
- 2x Xeon Gold 5122 (Dual Socket)

Both of these returned different results, the dual socket system returned the same value regardless of socket. So no you can't use silicon quality as a fingerprint.
