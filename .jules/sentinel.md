## 2024-05-18 - [Denial of Service via Unbounded Parameters]
**Vulnerability:** The CaptchaBuilder accepted unbounded integers for `length`, `text`, `interference_lines`, `interference_ellipses`, and `distortion`. An attacker could request a CAPTCHA with millions of characters or interference lines, causing the server to run out of memory or CPU and leading to a Denial of Service (DoS) vulnerability.
**Learning:** Even if a library handles visual output, any input defining iterations or allocations MUST be strictly bounded. Attackers often exploit configuration parameters that are mapped to loop counts.
**Prevention:** Always use `clamp()` or `.take()` to enforce strict maximum limits on any user-controllable input that dictates processing time or memory allocation size.
