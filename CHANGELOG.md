# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.11 (2024-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 87 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #15 from VilNeo/patch-1 ([`090c8ca`](https://github.com/samirdjelal/captcha-rs/commit/090c8ca9a1a534eaae59385d55f911b928aa301f))
    - Randomize noise ([`d882714`](https://github.com/samirdjelal/captcha-rs/commit/d88271439cd26440869152117ed7e92a2be7b954))
</details>

## v0.2.10 (2023-06-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 5 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #13 from samirdjelal/release_v0.2.10 ([`d7ce9a4`](https://github.com/samirdjelal/captcha-rs/commit/d7ce9a4dea0be61501ef1720ed5d4095dde96f88))
    - :bookmark: Release v0.2.10 ([`97c2289`](https://github.com/samirdjelal/captcha-rs/commit/97c2289800c034c73b0508cacea68c93409c20b0))
    - Merge pull request #12 from tedmielczarek-fastly/image-features ([`669647e`](https://github.com/samirdjelal/captcha-rs/commit/669647e63bdfd4898d47ace55e58a0c099f8973e))
    - Disable default features for image crate ([`1e3c85a`](https://github.com/samirdjelal/captcha-rs/commit/1e3c85a2adcd7d5e1ee8ad0f02b54c813bc1498a))
</details>

## v0.2.9 (2023-05-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 74 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #11 from samirdjelal/fix/compression_setter ([`3ce48cd`](https://github.com/samirdjelal/captcha-rs/commit/3ce48cdf6bdb158f8fb174d017c5f0080c47c421))
    - :bug: Fix: compression setter ([`5135089`](https://github.com/samirdjelal/captcha-rs/commit/51350895935d6e9ea04b5b48862e90b7e565542a))
    - :bug: Fix: compression setter ([`d5ecdb6`](https://github.com/samirdjelal/captcha-rs/commit/d5ecdb6b475767d5f72011e4ea8dc9932f41bfc5))
    - :sparkles: Add possibility to change Jpeg compression rate ([`07376c0`](https://github.com/samirdjelal/captcha-rs/commit/07376c0b06d656a040f6098ea2a26cf9ac7972c9))
    - Update README.md ([`98eba35`](https://github.com/samirdjelal/captcha-rs/commit/98eba35c10f549884b0af7e470ce3740314aae48))
    - Merge pull request #9 from EtZeta/patch-1 ([`a19f5a8`](https://github.com/samirdjelal/captcha-rs/commit/a19f5a8d8c1ae08df88745211fb11e9e83cc9fad))
    - Update README.md ([`cf6e9f8`](https://github.com/samirdjelal/captcha-rs/commit/cf6e9f86c229bca33835b523ec0f5a764a6ab45d))
</details>

## v0.2.7 (2023-03-13)

### Other

 - <csr-id-5d771569b992295c1cb78ae7b8e7647d6d3831db/> only run noise when complexity is above 1
   running the noise algorithms makes generation ~5 to 6 times slower

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 68 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Releasing v0.2.7 ([`b9301d7`](https://github.com/samirdjelal/captcha-rs/commit/b9301d7c5740e97266736319164d6dd2254275dd))
    - Fixing the tests ([`41fbd29`](https://github.com/samirdjelal/captcha-rs/commit/41fbd2986ef6fe1523d1226f312f62ab16da02d2))
    - Merge pull request #8 from Le0Developer/feat/expose-raw-image ([`3481b24`](https://github.com/samirdjelal/captcha-rs/commit/3481b246f707dfe279e352d237e11055996201d9))
    - Merge branch 'main' into feat/expose-raw-image ([`5a261a9`](https://github.com/samirdjelal/captcha-rs/commit/5a261a904fc1ce5e1f8bed7a0d2b0239a7e8ce0b))
    - Merge pull request #6 from Le0Developer/feat/custom-text-optimization ([`5d69f0c`](https://github.com/samirdjelal/captcha-rs/commit/5d69f0c283ea01ed7c5bee5402ef7b88992cb973))
    - Update example in readme ([`2c65743`](https://github.com/samirdjelal/captcha-rs/commit/2c65743b0e384a98aced1bee4d038b7b65d9570f))
    - Expose raw image, change to jpeg instead of png, remove Captcha::new ([`2cdc1d7`](https://github.com/samirdjelal/captcha-rs/commit/2cdc1d7394822cc0f37fccf27fb81837f0a95d57))
    - Only run noise when complexity is above 1 ([`5d77156`](https://github.com/samirdjelal/captcha-rs/commit/5d771569b992295c1cb78ae7b8e7647d6d3831db))
    - Add custom text support ([`f23bf89`](https://github.com/samirdjelal/captcha-rs/commit/f23bf89bbb85a735ede87f5b12e255286ca1c8ba))
    - Merge remote-tracking branch 'origin/main' into main ([`58e6601`](https://github.com/samirdjelal/captcha-rs/commit/58e6601dca40175701c8bbc0de0ee661875a39ef))
    - FUNDING.yml ([`afb219b`](https://github.com/samirdjelal/captcha-rs/commit/afb219b89167946eaa65647b3b1a8cd0b286b865))
    - Merge pull request #4 from samirdjelal/refactor/bg_and_text_color_changes ([`21b892e`](https://github.com/samirdjelal/captcha-rs/commit/21b892ee53bf527f5197e9c3a5a5cea86b4e10f2))
    - 🔀 Refactor: bg and text color changes ([`0404383`](https://github.com/samirdjelal/captcha-rs/commit/0404383d1fd2973ad33eaaede8a0352f9cb6cd5a))
    - 🔀 Refactor: bg and text color changes ([`84a960c`](https://github.com/samirdjelal/captcha-rs/commit/84a960ccfcb46c9a87d228add0191c755ff11d30))
    - Font scale ([`583161a`](https://github.com/samirdjelal/captcha-rs/commit/583161abfd98370b3130e9412f389972281e6cfb))
    - Font scale ([`9d0b8fc`](https://github.com/samirdjelal/captcha-rs/commit/9d0b8fc99f0f0ce55935b994cb833f662b6087f5))
    - Readme ([`7501750`](https://github.com/samirdjelal/captcha-rs/commit/75017503b2f2a0b5230ba141e921e2a9a2352401))
    - ✨ Feature: captcha complexity with noise ([`3ba7806`](https://github.com/samirdjelal/captcha-rs/commit/3ba7806810419e26bd6a4bcf032332ae2754eb56))
    - ✨ Feature: captcha complexity with noise ([`d82af07`](https://github.com/samirdjelal/captcha-rs/commit/d82af0750a3192784f4265a82f2d16e3cccdfcda))
    - Merge pull request #3 from samirdjelal/dev ([`b758654`](https://github.com/samirdjelal/captcha-rs/commit/b7586541ad5c8550d5558842a246503b7de8d36f))
    - ✨ Feature: captcha complexity with noise ([`b935c19`](https://github.com/samirdjelal/captcha-rs/commit/b935c19dadca9fff2b592d60a0630362f996e402))
    - 🐞 Fix: README.md ([`95bbdcf`](https://github.com/samirdjelal/captcha-rs/commit/95bbdcf88584333dbea2e1efc7431ce184c7df58))
    - Merge pull request #2 from samirdjelal/dev ([`322b982`](https://github.com/samirdjelal/captcha-rs/commit/322b98298ad24aa8dee48ed4906ef2835d9b34a3))
    - ✨ Feature: CaptchaBuilder. ([`a2c2b49`](https://github.com/samirdjelal/captcha-rs/commit/a2c2b49daf6d13787d1d7a93d01b3a19175a5e6a))
    - Ci ([`88957da`](https://github.com/samirdjelal/captcha-rs/commit/88957da6a9987d105b047760db8de8c7b593d643))
    - Update dependencies ([`3aca67c`](https://github.com/samirdjelal/captcha-rs/commit/3aca67c68d12ef9fe6ff48d8b6ef76d22c3681a3))
    - Update dependencies ([`fe8d923`](https://github.com/samirdjelal/captcha-rs/commit/fe8d9237ae20aa4c215e498f4cf6043bcc60398d))
    - Update dependencies ([`e3f60d0`](https://github.com/samirdjelal/captcha-rs/commit/e3f60d0ebfbe29fd260b95f96280959306bc17e1))
    - Update dependencies ([`54a048d`](https://github.com/samirdjelal/captcha-rs/commit/54a048d6cff02e66d01efcd929637afd4371a935))
    - Update dependencies ([`6d64b26`](https://github.com/samirdjelal/captcha-rs/commit/6d64b263c33895eab473120400abc242c164c5c6))
    - CI ([`13887de`](https://github.com/samirdjelal/captcha-rs/commit/13887de18e1dcfabeaf5bce9aacf2334da111617))
    - README.md ([`e82bcb3`](https://github.com/samirdjelal/captcha-rs/commit/e82bcb314510fadd185559a6dd84454d09a79286))
    - README.md ([`8eddc58`](https://github.com/samirdjelal/captcha-rs/commit/8eddc5848c150bb69c643a36ad52c0de399f6cdc))
    - .editorconfig ([`67523eb`](https://github.com/samirdjelal/captcha-rs/commit/67523eb5ded284816abbcbab55c891206db7e3cd))
    - Rename main.yml to CI.yml ([`363a88a`](https://github.com/samirdjelal/captcha-rs/commit/363a88a354bae40dd399cd1edfb7611ee7b6174d))
    - Merge pull request #1 from samirdjelal/dev ([`649d31f`](https://github.com/samirdjelal/captcha-rs/commit/649d31f39ca8003c8d283492897f03581c97a125))
    - Edit main.yml ([`65f6e80`](https://github.com/samirdjelal/captcha-rs/commit/65f6e809912ca794ad6db7039b36e12a635b68b5))
    - Update Jenkinsfile ([`fa5469a`](https://github.com/samirdjelal/captcha-rs/commit/fa5469a30a7c4e5aeea8e86990837c6387c48747))
    - Added Jenkinsfile ([`2349ec5`](https://github.com/samirdjelal/captcha-rs/commit/2349ec5353cb99a1b40af5132eb1135cba7ad853))
    - Added Jenkinsfile ([`267f2bf`](https://github.com/samirdjelal/captcha-rs/commit/267f2bf77fb50166df5b7df390978e2b27f79429))
    - Added Jenkinsfile ([`62598d1`](https://github.com/samirdjelal/captcha-rs/commit/62598d1a926b89a69031f03df57c9c529afdc3cf))
    - Added Jenkinsfile ([`a7bdf37`](https://github.com/samirdjelal/captcha-rs/commit/a7bdf3745618274400b0ab8412a94528468db3a7))
    - 📃 README.md ([`9b086eb`](https://github.com/samirdjelal/captcha-rs/commit/9b086eb8630e661f89ef912c2d5502bdb273b2df))
    - 📃 README.md ([`fc1789f`](https://github.com/samirdjelal/captcha-rs/commit/fc1789f802c25c4277bc38d8bbf6b44994759a21))
    - Update README.md ([`49104f1`](https://github.com/samirdjelal/captcha-rs/commit/49104f111e5b656f58e6ed65680db037018e7125))
    - 📃 README.md ([`84c6a9f`](https://github.com/samirdjelal/captcha-rs/commit/84c6a9f92eec78e0c078d811495c417c209c3d93))
    - 📃 README.md ([`e8728bd`](https://github.com/samirdjelal/captcha-rs/commit/e8728bda3f2feae788cdb2cde56bef9645f8cafc))
    - 🐞 Fix: test not working correctly ([`3416661`](https://github.com/samirdjelal/captcha-rs/commit/3416661641ddb202b98c5ab8f83078bd13d6deb6))
    - 🐞 Fix: test not working correctly ([`8dd2674`](https://github.com/samirdjelal/captcha-rs/commit/8dd26743cee5b8a034dd44cb32122e603fcd0ad2))
    - Update and rename master.yml to main.yml ([`656420a`](https://github.com/samirdjelal/captcha-rs/commit/656420a711d233ab462449dee7dcfb9e2e33c6d9))
    - Merge remote-tracking branch 'origin/main' into main ([`7e446cd`](https://github.com/samirdjelal/captcha-rs/commit/7e446cd2f947059297ca0590883757d340da9700))
    - README.md ([`72f6759`](https://github.com/samirdjelal/captcha-rs/commit/72f675932490675b8d552f1fd8825e25b9505ff7))
    - Create master.yml ([`aa8dab4`](https://github.com/samirdjelal/captcha-rs/commit/aa8dab4c68c44493d13b4f6d375374872056c701))
    - README.md ([`d1b644c`](https://github.com/samirdjelal/captcha-rs/commit/d1b644c75bb335cde0027335b5d512030931f08a))
    - README.md ([`2560ab5`](https://github.com/samirdjelal/captcha-rs/commit/2560ab5228d5b886fbc31c01489fd06f3f75f6f1))
    - Create FUNDING.yml ([`66eff29`](https://github.com/samirdjelal/captcha-rs/commit/66eff29bf1cf98c40adc67ef51724415e44f6f41))
    - README.md ([`8718363`](https://github.com/samirdjelal/captcha-rs/commit/8718363d0ad2268b4490201295698e81e8e34cee))
    - README.md ([`a43e047`](https://github.com/samirdjelal/captcha-rs/commit/a43e047457df7354710473524976e4ec319e3281))
    - MIT license ([`d747cf6`](https://github.com/samirdjelal/captcha-rs/commit/d747cf6fc6fe4e93744ed392f999caa847bedc50))
    - Edit docs ([`9feb9fb`](https://github.com/samirdjelal/captcha-rs/commit/9feb9fbe96a2fc50f3752c84be59bb2da9ec65d5))
    - Update README.md ([`d8f2c5e`](https://github.com/samirdjelal/captcha-rs/commit/d8f2c5e2c33e5ab39a427be569dc416c4b957eed))
    - Edit docs ([`8f74c0d`](https://github.com/samirdjelal/captcha-rs/commit/8f74c0d6f0d3a4d9720d1a7ce14da788f4b43c10))
    - Example added to lib.rs ([`15d362c`](https://github.com/samirdjelal/captcha-rs/commit/15d362cd1216027625566d4854ee51bf7d9448ec))
    - Example added to lib.rs ([`b523f4e`](https://github.com/samirdjelal/captcha-rs/commit/b523f4e4e20abbd247f4c096703b50d3fc87a14e))
    - Example added to lib.rs ([`5541b50`](https://github.com/samirdjelal/captcha-rs/commit/5541b50cb76c82cbb7e282d04c363009318ce8df))
    - Readme made ([`3b6a1e6`](https://github.com/samirdjelal/captcha-rs/commit/3b6a1e6963a42e5c2a9f46e89aca671b33163428))
    - Init ([`5e22fa2`](https://github.com/samirdjelal/captcha-rs/commit/5e22fa2c1bef143cb710ccd45ff46abb3c8ef12e))
    - Init ([`9fa0d9e`](https://github.com/samirdjelal/captcha-rs/commit/9fa0d9e48e0c6d425ace0fa9e26833089f3dfd4b))
</details>

