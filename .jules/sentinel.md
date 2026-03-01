## 2024-05-18 - [Denial of Service via Unbounded Parameters]
**Vulnerability:** The CaptchaBuilder accepted unbounded integers for `length`, `text`, `interference_lines`, `interference_ellipses`, and `distortion`. An attacker could request a CAPTCHA with millions of characters or interference lines, causing the server to run out of memory or CPU and leading to a Denial of Service (DoS) vulnerability.
**Learning:** Even if a library handles visual output, any input defining iterations or allocations MUST be strictly bounded. Attackers often exploit configuration parameters that are mapped to loop counts.
**Prevention:** Always use `clamp()` or `.take()` to enforce strict maximum limits on any user-controllable input that dictates processing time or memory allocation size.
## 2024-05-24 - Plaintext solution in JWT payload
**Vulnerability:** A JWT claim contained the plaintext CAPTCHA solution, which could be easily extracted by Base64 decoding the token.
**Learning:** Always use a hashed or encrypted payload for sensitive data stored in a JWT if its exposure compromises the security mechanism (e.g. CAPTCHAs, secrets).
**Prevention:** Store an irreversible hash (e.g., HMAC-SHA256 or SHA256 of the concatenated secret and solution) in the JWT payload instead of the plaintext data.
