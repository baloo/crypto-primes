//! Some known primes for testing, taken from <https://primes.utm.edu/lists>

use crypto_bigint::{U1024, U128, U256, U384, U512};

pub(crate) const PRIMES_128: &[U128] = &[
    U128::from_be_hex("000000087b57be17f0ecdbf18a227bd9"),
    U128::from_be_hex("000000038fc1442dfbcd26c084f48fed"),
    U128::from_be_hex("0000000694cc33f9e35619692c955d8b"),
    U128::from_be_hex("00000004941037627ac8fcd5974a9d63"),
    U128::from_be_hex("000000017607ef654eb9a13ffa163c75"),
    U128::from_be_hex("0000000775365eac5cbfb0c6e7157571"),
    U128::from_be_hex("000000072107e16cf097750c6b8ddb9b"),
    U128::from_be_hex("000000067c3ef2f455d8c8635668f2a3"),
    U128::from_be_hex("000000054060a750a88d007bd41db2cb"),
    U128::from_be_hex("000000038b689c351cf329d5efd5676b"),
];

pub(crate) const PRIMES_256: &[U256] = &[
    U256::from_be_hex(concat![
        "000000b5fbdee6707ad4aae136384de5",
        "bab269c2f0762d7f79e125a789aaeb0d",
    ]),
    U256::from_be_hex(concat![
        "000000595c108635b5d7a48618c1c266",
        "bdabd5b872848056fa2673cdb4a26d73",
    ]),
    U256::from_be_hex(concat![
        "00000119b6ec864440b587017783e3bd",
        "5cfe32376bf30fd88e78df6308a21fbf",
    ]),
    U256::from_be_hex(concat![
        "0000008dc92563d4d0e518083b120be1",
        "f972f0e6ec1742dcbb3c005d45d2e1e5",
    ]),
    U256::from_be_hex(concat![
        "000000da51fff5ef297699631f9a4cc2",
        "501c293cb63dcd0460d142edb1d3c347",
    ]),
    U256::from_be_hex(concat![
        "000000d903f6577d41bdad7337db1c21",
        "8f7c2db8ff2972659728ae5f2ffb2717",
    ]),
    U256::from_be_hex(concat![
        "0000009d297d4f202f33ee588bcdaa03",
        "9b851f99cc019cbb0a8ec7a39a22b7d9",
    ]),
    U256::from_be_hex(concat![
        "0000008bf47e7a8becb006495390462f",
        "1fbf78ec20ec06bd63a9c40c0c75302f",
    ]),
    U256::from_be_hex(concat![
        "00000162263cf603cfa774a2f1b9ce87",
        "3971228cdc785ce7a15ac33ae5a79eef",
    ]),
];

pub(crate) const PRIMES_384: &[U384] = &[
    U384::from_be_hex(concat![
        "00000efd19f2e8e87c453b59401661bb",
        "58f97b1ea71949ea3ae7b31359bfc34e",
        "7739c6776eedea9771ce830d8185e20d",
    ]),
    U384::from_be_hex(concat![
        "0000047a095d446066c0b87c0275f2d2",
        "cab77570e1eb88cd4564f5d954859544",
        "e70927423c0f7c640cf2cbfa05bbdd0d",
    ]),
    U384::from_be_hex(concat![
        "00000a82ad1ecb13aacc42c1cbadab95",
        "9b9a1e7a3d0918f161a28dc9b2471ec0",
        "6f4891db344c75ee700103b5a1a2f3d1",
    ]),
    U384::from_be_hex(concat![
        "000020c0b99c474406d68b8eb20b10ed",
        "3271374fd135e45a84cba9b48c875c65",
        "8962bf77e4212913e13c4f8a303347ef",
    ]),
    U384::from_be_hex(concat![
        "00000e0d00c96e00ccf3e3be34cd630b",
        "e9d6021c863846f5d07d2c71a69e3c84",
        "dbd8d7282d08e8a7d968050cc94e7bd9",
    ]),
    U384::from_be_hex(concat![
        "00000d2b056abc94061a4dc14df36500",
        "3e29291a5dbc939084e4646930592bde",
        "910ed5e8a777e855291866c0ec9f06fb",
    ]),
    U384::from_be_hex(concat![
        "000005ff23a9608f41eee7c33f36349a",
        "995b653ffa1a65ebba86506ff17b26fa",
        "84d124dc7901025d7fb553551b07dec7",
    ]),
    U384::from_be_hex(concat![
        "0000047024e9ce77cd142ca88c8b7dd4",
        "697ec6fe502f8aac8d7e6f730b6a425c",
        "73b7279b953afb8799604c0c9a6e55bd",
    ]),
    U384::from_be_hex(concat![
        "00000f70a1f5fee022508a98396bd5df",
        "4710666d054a2ae914f862724527f449",
        "3560cc2a54b465de06fab4cac865c7af",
    ]),
    U384::from_be_hex(concat![
        "000006a240c418491a3f4782922cb2fe",
        "f8b75a70a6514cb5ffec4e75a76762d3",
        "e0f657debbc91822a4c1e1e1e64e8015",
    ]),
];

pub(crate) const PRIMES_512: &[U512] = &[
    U512::from_be_hex(concat![
        "000335b8541114f93d1b78040c89eda3",
        "a73c8e9d9b5d2a1587e6ba20ca21c18b",
        "ca58158f28efc68cb8888c9b0469163e",
        "bb21169e7cce71914c0c6c232d79477d",
    ]),
    U512::from_be_hex(concat![
        "00010009768f0beb02a44f7763e79a47",
        "f8a87f0cbfa75b8b1fce94140c527686",
        "9aadf214adb09778bd6d75254ef1e050",
        "4310a80ca5ed40f4a5690c55f4c79ed3",
    ]),
    U512::from_be_hex(concat![
        "000322a68577f54915462d72d611446d",
        "bc5be09af18bbae8601b9a0ed8f84648",
        "b38281cd0dbc52841c12dcf0fc7a5eb2",
        "be91aa71be6ad215ff33421caeeedfd5",
    ]),
    U512::from_be_hex(concat![
        "0002b08a9ef85889a89b7683834196df",
        "1cabfbd95d588e55e445e17f60316c6b",
        "7561efe7865fb2292ad310cd48c6f24d",
        "5e6c666c91ed5b69ea277ba1389ce9e1",
    ]),
    U512::from_be_hex(concat![
        "0002d9ac8adbb1060dad08adf66eba42",
        "d395889862bd3a81cd7ab82c8dfa83a4",
        "149ec8ab74365844e3a8a1222fa42890",
        "eb4bd956ae3d8fe80e8f666a9f47fa81",
    ]),
    U512::from_be_hex(concat![
        "00025ad74b372513204d5811dabf8502",
        "0e01176f281b8347133d90f230b0922f",
        "b9a79c9c99cc233a6c8d993ad0418c7a",
        "fa61043745ccf6bcd4a44b383c2b3e39",
    ]),
    U512::from_be_hex(concat![
        "00016cd147e818a7ea811934c3b8d814",
        "405504a1163cba54cb0d74c618adf584",
        "6ca8edb54aac6b8deb3ceb32e4b86b4c",
        "6cf0f4e02b613b46ddd983b7ba453733",
    ]),
    U512::from_be_hex(concat![
        "0001d5eff72acadc9097b81893e8366d",
        "195aa19ada4ad143b3abc07f34da8e84",
        "a376dbd052ad1892e6f9e8834b641dfc",
        "9e48c173e8d9c93b93ab0a49bf5122a9",
    ]),
    U512::from_be_hex(concat![
        "000210a0f21482258ef7025fbc38f715",
        "2e048972ded3fdb50e00003db347e096",
        "feb75ae35f7dcec50938ee38b1f0b6c6",
        "e85a8d83186628c612b6155910b8042d",
    ]),
    U512::from_be_hex(concat![
        "00029bef465fdcab6dbd9391c771e336",
        "e6479c7601301e46e1a2796a50e76f5f",
        "2a1f39e5030fab040486c3425a02218e",
        "08038192f59c467e6e7b97737b6a9707",
    ]),
];

pub(crate) const PRIMES_1024: &[U1024] = &[
    U1024::from_be_hex(concat![
        "00000004df72d07b4b71c8dacb6cffa9",
        "54f8d88254b6277099308baf003fab73",
        "227f34029643b5a263f66e0d3c3fa297",
        "ef71755efd53b8fb6cb812c6bbf7bcf1",
        "79298bd9947c4c8b14324140a2c0f5fa",
        "d7958a69050a987a6096e9f055fb38ed",
        "f0c5889eca4a0cfa99b45fbdeee4c696",
        "b328ddceae4723945901ec025076b12b",
    ]),
    U1024::from_be_hex(concat![
        "0000000cb50e82a8583f44ee0025942e",
        "7362991b24e12663a0ddc234a57b0f7b",
        "4ff7b025bf5a6707dedc2898e70b7390",
        "42c95a996283dffdf67558768784553c",
        "61e302e8812bc90f0bb0696870cfb910",
        "b560cefed8d99bbf7a00b31ccdbd56f3",
        "594e5a653cfd127d2167b13119e5c45c",
        "3f76b4e3d904a9bc0cbb43c33aa7f23b",
    ]),
    U1024::from_be_hex(concat![
        "00000007a364ab3de755f924642bd527",
        "3524234f78395da1ed9098f39af4fe24",
        "8288b0cb7f1c27214588969479d7dc9f",
        "0d327b5544dd4c095aa1fa271df421fe",
        "9ee460855cc8423d223e2c85dc793f6b",
        "abdca7fc804ea1f408f867db053bfd98",
        "c45085ea5d805c78d2863bacdfcaf4c6",
        "147ebb74a9056045074785714c0b84ed",
    ]),
    U1024::from_be_hex(concat![
        "00000005fc772f3f3fea4690a02506ad",
        "5813a9e05abcc3282fe4b074a410925b",
        "a675a980089de5e80391ae6d87b69987",
        "08c9071b5c542db3759da65d625e54cf",
        "4edd5117c32b465264104d834585af50",
        "7e7acbf1bfb3bc9dd64ba6c37259648b",
        "5667d2435344d427a9ce17b1b9eaa4d0",
        "8ff8ff2b4f5cbf92d676629941157ca5",
    ]),
    U1024::from_be_hex(concat![
        "00000006ef35851220469eae6510a0fa",
        "2c8d8952142b178c7b8003297878267e",
        "87d9ce27c11dc0143b84b6892f13d39f",
        "92bf29acf5605feed5b14eba36ecce31",
        "bfe0eb37e3040fd1b616bf5883da20e7",
        "6237a823ac11dc6ecd739324a59e6dda",
        "8e402bcc6e9a4ac43b3e51f5e532ae04",
        "11e01f4abb50178cdb73023cdd71d6f1",
    ]),
];

/// A Cunningham chain (of the first kind) is a sequence of `k` numbers
/// such that `n_{k+1} = n{k} * 2 + 1`, and each `n_k` is prime.
/// We provide pairs `(k, n_0)` here.
pub(crate) const CUNNINGHAM_CHAINS_128: &[(usize, U128)] = &[
    (17, U128::from_be_hex("00000000000000959c603768458440ef")),
    (16, U128::from_be_hex("0000000000001355a29aa8a1159d827f")),
];

pub(crate) const CUNNINGHAM_CHAINS_512: &[(usize, U512)] = &[
    (
        14,
        U512::from_be_hex(concat![
            "00000000000000000000000000000000",
            "00000000000000015554e93d3f6ddebb",
            "ba232f8e5d8f9fed9c5febd7cd293977",
            "0cee48c95a5fd0af9aea46743d782bff"
        ]),
    ),
    (
        13,
        U512::from_be_hex(concat![
            "00000000000000000000000000000000",
            "000000015d2866f53c7c1d8895fc5d80",
            "c3c12d1304cacffd7f1b2a543881d0b0",
            "3849948d12793a3e8b60f74933ccc1ff"
        ]),
    ),
];
