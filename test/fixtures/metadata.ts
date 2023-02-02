export const getOnchainMetadata = [
  // invalid
  {
    name: 'invalid on-chain metadata version 1 (name is array and should be string)',
    data: {
      asset:
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd4249534f4e',
      policy_id: '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd',
      asset_name: '4249534f4e',
      quantity: '1000000000000000',
      initial_mint_tx_hash:
        '96985bee7d57000e70ab444fe44e94767ef6dc1ccc721e46b026651cd5566d69',
      mint_or_burn_count: '1',
      onchain_metadata: {
        version: '1.0',
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd': {
          BISON: {
            name: ['Bison Coin'],
            files: [
              {
                src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
                name: 'BISON',
                mediaType: 'image/png',
              },
            ],
            image: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            symbol: 'BISON',
            minting: {
              type: 'time-lock-policy',
              blockchain: 'cardano',
              mintedBeforeSlotNumber: 45933708,
            },
            mediaType: 'image/png',
            tokenType: 'token',
            description:
              'Bison Coin is a decentralized meme coin on cardano network.',
            totalSupply: 1000000000000000,
          },
        },
      },
      onchain_metadata_cbor:
        'a11902d1a278383137373433343332343136383065346461656637636266653335333666633835376365323366623636636430623636333230623265336464a1654249534f4ea96b6465736372697074696f6e783b4269736f6e20436f696e206973206120646563656e7472616c697a6564206d656d6520636f696e206f6e2063617264616e6f206e6574776f726b2e6566696c657381a3696d656469615479706569696d6167652f706e67646e616d65654249534f4e637372637835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e766465696d6167657835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e7664696d656469615479706569696d6167652f706e67676d696e74696e67a36a626c6f636b636861696e6763617264616e6f766d696e7465644265666f7265536c6f744e756d6265721a02bce48c64747970657074696d652d6c6f636b2d706f6c696379646e616d656a4269736f6e20436f696e6673796d626f6c654249534f4e69746f6b656e5479706565746f6b656e6b746f74616c537570706c791b00038d7ea4c680006776657273696f6e63312e30',
      metadata: null,
    },
    response: {
      onchainMetadata: {
        name: ['Bison Coin'],
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
        image: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
        symbol: 'BISON',
        minting: {
          type: 'time-lock-policy',
          blockchain: 'cardano',
          mintedBeforeSlotNumber: 45933708,
        },
        mediaType: 'image/png',
        tokenType: 'token',
        description:
          'Bison Coin is a decentralized meme coin on cardano network.',
        totalSupply: 1000000000000000,
      },
      validCIPversion: null,
    },
  },
  {
    name: 'invalid on-chain metadata version 1 (image is number and should be string or array of strings)',
    data: {
      asset:
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd4249534f4e',
      policy_id: '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd',
      asset_name: '4249534f4e',
      quantity: '1000000000000000',
      initial_mint_tx_hash:
        '96985bee7d57000e70ab444fe44e94767ef6dc1ccc721e46b026651cd5566d69',
      mint_or_burn_count: '1',
      onchain_metadata: {
        version: '1.0',
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd': {
          BISON: {
            name: 'Bison Coin',
            files: [
              {
                src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
                name: 'BISON',
                mediaType: 'image/png',
              },
            ],
            image: 1,
            symbol: 'BISON',
            minting: {
              type: 'time-lock-policy',
              blockchain: 'cardano',
              mintedBeforeSlotNumber: 45933708,
            },
            mediaType: 'image/png',
            tokenType: 'token',
            description:
              'Bison Coin is a decentralized meme coin on cardano network.',
            totalSupply: 1000000000000000,
          },
        },
      },
      // onchain_metadata_cbor don't mater
      onchain_metadata_cbor:
        'a11902d1a278383137373433343332343136383065346461656637636266653335333666633835376365323366623636636430623636333230623265336464a1654249534f4ea96b6465736372697074696f6e783b4269736f6e20436f696e206973206120646563656e7472616c697a6564206d656d6520636f696e206f6e2063617264616e6f206e6574776f726b2e6566696c657381a3696d656469615479706569696d6167652f706e67646e616d65654249534f4e637372637835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e766465696d6167657835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e7664696d656469615479706569696d6167652f706e67676d696e74696e67a36a626c6f636b636861696e6763617264616e6f766d696e7465644265666f7265536c6f744e756d6265721a02bce48c64747970657074696d652d6c6f636b2d706f6c696379646e616d656a4269736f6e20436f696e6673796d626f6c654249534f4e69746f6b656e5479706565746f6b656e6b746f74616c537570706c791b00038d7ea4c680006776657273696f6e63312e30',
      metadata: null,
    },
    response: {
      validCIPversion: null,
      onchainMetadata: {
        name: 'Bison Coin',
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
        image: 1,
        symbol: 'BISON',
        minting: {
          type: 'time-lock-policy',
          blockchain: 'cardano',
          mintedBeforeSlotNumber: 45933708,
        },
        mediaType: 'image/png',
        tokenType: 'token',
        description:
          'Bison Coin is a decentralized meme coin on cardano network.',
        totalSupply: 1000000000000000,
      },
    },
  },
  // valid
  {
    name: 'valid on-chain metadata version 1',
    data: {
      asset:
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd4249534f4e',
      policy_id: '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd',
      asset_name: '4249534f4e',
      quantity: '1000000000000000',
      initial_mint_tx_hash:
        '96985bee7d57000e70ab444fe44e94767ef6dc1ccc721e46b026651cd5566d69',
      mint_or_burn_count: '1',
      onchain_metadata: {
        version: '1.0',
        '1774343241680e4daef7cbfe3536fc857ce23fb66cd0b66320b2e3dd': {
          BISON: {
            name: 'Bison Coin',
            files: [
              {
                src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
                name: 'BISON',
                mediaType: 'image/png',
              },
            ],
            image: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            symbol: 'BISON',
            minting: {
              type: 'time-lock-policy',
              blockchain: 'cardano',
              mintedBeforeSlotNumber: 45933708,
            },
            mediaType: 'image/png',
            tokenType: 'token',
            description:
              'Bison Coin is a decentralized meme coin on cardano network.',
            totalSupply: 1000000000000000,
          },
        },
      },
      onchain_metadata_cbor:
        'a11902d1a278383137373433343332343136383065346461656637636266653335333666633835376365323366623636636430623636333230623265336464a1654249534f4ea96b6465736372697074696f6e783b4269736f6e20436f696e206973206120646563656e7472616c697a6564206d656d6520636f696e206f6e2063617264616e6f206e6574776f726b2e6566696c657381a3696d656469615479706569696d6167652f706e67646e616d65654249534f4e637372637835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e766465696d6167657835697066733a2f2f516d506b3653593850347957656b4b315a394253724c665138625044485a696972575667693568647379766e7664696d656469615479706569696d6167652f706e67676d696e74696e67a36a626c6f636b636861696e6763617264616e6f766d696e7465644265666f7265536c6f744e756d6265721a02bce48c64747970657074696d652d6c6f636b2d706f6c696379646e616d656a4269736f6e20436f696e6673796d626f6c654249534f4e69746f6b656e5479706565746f6b656e6b746f74616c537570706c791b00038d7ea4c680006776657273696f6e63312e30',
      metadata: null,
    },
    response: {
      onchainMetadata: {
        name: 'Bison Coin',
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
        image: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
        symbol: 'BISON',
        minting: {
          type: 'time-lock-policy',
          blockchain: 'cardano',
          mintedBeforeSlotNumber: 45933708,
        },
        mediaType: 'image/png',
        tokenType: 'token',
        description:
          'Bison Coin is a decentralized meme coin on cardano network.',
        totalSupply: 1000000000000000,
      },
      validCIPversion: 'CIP25v1',
    },
  },
  {
    name: 'valid on-chain metadata version 2',
    data: {
      asset:
        'add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f58424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751',
      policy_id: 'add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5',
      asset_name:
        '8424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751',
      fingerprint: 'asset10m8zhjspkwczmmx86hq9m6gdqclzxjc494wm95',
      quantity: '10',
      initial_mint_tx_hash:
        '4cddc91fdfea357aa81f50be0c2d0cc839124eb9f664e8490eaa04ff9dc387a4',
      mint_or_burn_count: 1,
      onchain_metadata: {
        version: 2,
        '0xadd8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5': {
          '0x8424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751':
            {
              name: 'Optim EQT',
              image: [
                'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
                'di',
              ],
            },
        },
        '0xec4358d0daae8ab25facf91eff42ad60175476d620f6c22193176e02': {
          '0x8424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751':
            {
              name: 'Optim Borrower Token',
              image: [
                'ipfs://bafkreids5uegktmjz753qngxmjlttzrx7dg5qeqqsnf734csuijpkg24',
                'ne',
              ],
            },
        },
      },
      onchain_metadata_cbor:
        'a11902d1a36776657273696f6e02581cadd8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5a158208424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751a2646e616d65694f7074696d2045515465696d616765827840697066733a2f2f6261666b72656966356961706b737572707a6f656779786c376a7962646c786271737a3275707361677532646d6279676a3471626636636663626469581cec4358d0daae8ab25facf91eff42ad60175476d620f6c22193176e02a158208424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751a2646e616d65744f7074696d20426f72726f77657220546f6b656e65696d616765827840697066733a2f2f6261666b7265696473357565676b746d6a7a373533716e67786d6a6c74747a72783764673571657171736e66373334637375696a706b673234626e65',
      metadata: null,
    },
    response: {
      onchainMetadata: {
        name: 'Optim EQT',
        image: [
          'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
          'di',
        ],
      },
      validCIPversion: 'CIP25v2',
    },
  },
  {
    name: 'valid on-chain metadata version 2 with empty asset_name',
    data: {
      asset: 'add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5',
      policy_id: 'add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5',
      asset_name: '',
      fingerprint: 'asset10m8zhjspkwczmmx86hq9m6gdqclzxjc494wm95',
      quantity: '10',
      initial_mint_tx_hash:
        '4cddc91fdfea357aa81f50be0c2d0cc839124eb9f664e8490eaa04ff9dc387a4',
      mint_or_burn_count: 1,
      onchain_metadata: {
        version: 2,
        '0xadd8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5': {
          '0x': {
            name: 'Optim EQT',
            image: [
              'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
              'di',
            ],
          },
        },
      },
      // NOTE: this is hand crafted CBOR which replaces 32 bytes of asset name with 0 bytes
      onchain_metadata_cbor:
        'a11902d1a36776657273696f6e02581cadd8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5a15800a2646e616d65694f7074696d2045515465696d616765827840697066733a2f2f6261666b72656966356961706b737572707a6f656779786c376a7962646c786271737a3275707361677532646d6279676a3471626636636663626469581cec4358d0daae8ab25facf91eff42ad60175476d620f6c22193176e02a158208424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751a2646e616d65744f7074696d20426f72726f77657220546f6b656e65696d616765827840697066733a2f2f6261666b7265696473357565676b746d6a7a373533716e67786d6a6c74747a72783764673571657171736e66373334637375696a706b673234626e65',
      metadata: null,
    },
    response: {
      onchainMetadata: {
        name: 'Optim EQT',
        image: [
          'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
          'di',
        ],
      },
      validCIPversion: 'CIP25v2',
    },
  },
  {
    name: 'valid on-chain metadata version 2 with empty asset_name (preview asset)',
    data: {
      asset: '1b6ed7ba4c9671b8b798af6eff7146396c7b04dc12a6463c6a29be85',
      policy_id: '1b6ed7ba4c9671b8b798af6eff7146396c7b04dc12a6463c6a29be85',
      asset_name: null,
      fingerprint: 'asset1q5luj5axz3m2wx3pt9mkpfnqcyhz7evnj445te',
      quantity: '1000000',
      initial_mint_tx_hash:
        'ce471569a87aedcdc51f84e98c008bf6851d0dbe760ededaaea7f017ac67b9e1',
      mint_or_burn_count: 1,
      onchain_metadata: {
        version: '2.0',
        '0x1b6ed7ba4c9671b8b798af6eff7146396c7b04dc12a6463c6a29be85': {
          '0x': {
            name: '',
            image: 'ipfs://todo',
            description: 'Change the world with FinanceBinaries!',
          },
        },
      },
      onchain_metadata_cbor:
        'a11902d1a2581c1b6ed7ba4c9671b8b798af6eff7146396c7b04dc12a6463c6a29be85a140a36b6465736372697074696f6e78264368616e67652074686520776f726c6420776974682046696e616e636542696e61726965732165696d6167656b697066733a2f2f746f646f646e616d65606776657273696f6e63322e30',
      metadata: null,
    },
    response: {
      onchainMetadata: {
        name: '',
        image: 'ipfs://todo',
        description: 'Change the world with FinanceBinaries!',
      },
      validCIPversion: 'CIP25v2',
    },
  },
  {
    name: 'invalid on-chain metadata version 2 with policy and asset_name encoded as string',
    data: {
      asset:
        'd12d8c05c03484409f157917f21b323824d892130e4085006eaefc4a5041524131323933',
      policy_id: 'd12d8c05c03484409f157917f21b323824d892130e4085006eaefc4a',
      asset_name: '5041524131323933',
      quantity: '1',
      initial_mint_tx_hash:
        '2edd088027b6d7c77cff4e0d580ae2764d56f31e4df1ba979b571dc8005e4a05',
      mint_or_burn_count: '1',
      onchain_metadata: {
        version: '2.0',
        d12d8c05c03484409f157917f21b323824d892130e4085006eaefc4a: {
          PARA0003: {
            body: 'Tan normal',
            eyes: 'Looking down',
            hair: 'Gerald',
            name: 'ParaPains #0003',
            files: [
              {
                src: 'ipfs://QmdrL7XUP8jBSzTzw7HSJGxRDwgEmYtk5obJqdi9f62mYC',
                name: 'ParaPains #0003',
                mediaType: 'image/jpg',
              },
            ],
            image: 'ipfs://QmdrL7XUP8jBSzTzw7HSJGxRDwgEmYtk5obJqdi9f62mYC',
            mouth: 'Buck tooth',
            clothes: 'Dursley knit',
            website: 'https://www.painsnft.com',
            eyebrows: 'Blue',
            mediaType: 'image/jpg',
            background: 'Cream',
            accessories: 'Diamond hands',
          },
          PARA1293: {
            body: 'Tan button',
            eyes: 'Chicken eyes',
            hair: 'Purple fro',
            name: 'ParaPains #1293',
            files: [
              {
                src: 'ipfs://QmdhYyHuCXd2CmFo5p5cf5p2ajPLimCWn96WtdNiRVwwJZ',
                name: 'ParaPains #1293',
                mediaType: 'image/jpg',
              },
            ],
            image: 'ipfs://QmdhYyHuCXd2CmFo5p5cf5p2ajPLimCWn96WtdNiRVwwJZ',
            mouth: 'Blue smirk',
            clothes: 'Charlie brown',
            website: 'https://www.painsnft.com',
            eyebrows: 'Blue',
            mediaType: 'image/jpg',
            background: 'Mid grey',
            accessories: 'Happy balloon',
          },
          PARA1802: {
            body: 'Yellow button',
            eyes: 'Chilled',
            hair: 'Rapunzel',
            name: 'ParaPains #1802',
            files: [
              {
                src: 'ipfs://Qmcbs6dd4z52KdjVbU3sHRDTCv6B3vxrbVHTfewXduGK1z',
                name: 'ParaPains #1802',
                mediaType: 'image/jpg',
              },
            ],
            image: 'ipfs://Qmcbs6dd4z52KdjVbU3sHRDTCv6B3vxrbVHTfewXduGK1z',
            mouth: 'Shocked',
            clothes: 'Maroon T',
            website: 'https://www.painsnft.com',
            eyebrows: 'Worried brown',
            mediaType: 'image/jpg',
            background: 'Deep green',
            accessories: 'None',
          },
          PARA1913: {
            body: 'Purple broken',
            eyes: 'Bloodshot',
            hair: 'Yellow basquiat',
            name: 'ParaPains #1913',
            files: [
              {
                src: 'ipfs://QmYPqwszoE3zZtupPADDMT1aZCD4XBP93Aa5qA6eqAHpM2',
                name: 'ParaPains #1913',
                mediaType: 'image/jpg',
              },
            ],
            image: 'ipfs://QmYPqwszoE3zZtupPADDMT1aZCD4XBP93Aa5qA6eqAHpM2',
            mouth: 'Sad blue',
            clothes: 'Hawaiian shirt',
            website: 'https://www.painsnft.com',
            eyebrows: 'Blue',
            mediaType: 'image/jpg',
            background: 'Cream',
            accessories: 'None',
          },
        },
      },
      metadata: null,
    },
    response: {
      validCIPversion: null,
      onchainMetadata: {
        name: 'ParaPains #1293',
        image: 'ipfs://QmdhYyHuCXd2CmFo5p5cf5p2ajPLimCWn96WtdNiRVwwJZ',
        body: 'Tan button',
        eyes: 'Chicken eyes',
        hair: 'Purple fro',
        files: [
          {
            src: 'ipfs://QmdhYyHuCXd2CmFo5p5cf5p2ajPLimCWn96WtdNiRVwwJZ',
            name: 'ParaPains #1293',
            mediaType: 'image/jpg',
          },
        ],
        mouth: 'Blue smirk',
        clothes: 'Charlie brown',
        website: 'https://www.painsnft.com',
        eyebrows: 'Blue',
        mediaType: 'image/jpg',
        background: 'Mid grey',
        accessories: 'Happy balloon',
      },
    },
  },
  {
    name: 'invalid on-chain metadata version 1 with invalid onchain_metadata structure',
    data: {
      asset:
        '9ed0d6254917ec44fb7368c034324a009e86b339a4dfab8f67dc5e58546865204c6567656e64',
      policy_id: '9ed0d6254917ec44fb7368c034324a009e86b339a4dfab8f67dc5e58',
      asset_name: '546865204c6567656e64',
      quantity: '2',
      initial_mint_tx_hash:
        'b06cfcd74cdb80789ef2b0731676eb76173f7384bc7e9a86a88db11ae6f2d2b1',
      mint_or_burn_count: '2',
      onchain_metadata: {
        '9ed0d6254917ec44fb7368c034324a009e86b339a4dfab8f67dc5e58': {
          thelegend: {
            id: '1',
            name: 'The Legend',
            image: 'ipfs://QmTzQarLF7sC2YhcLMbDnmtjLeunB6WuMrTqQGUC2XJozh',
            Collection: 'The Saloon Collection',
            description: 'The Head Bouncer',
          },
        },
      },
      metadata: null,
    },
    response: {
      validCIPversion: null,
      onchainMetadata: null,
    },
  },
];

export const validateCIP68Metadata = [
  {
    name: 'Matrix Berry #99 (NFT metadata)',
    payload: {
      metadata: {
        description: '',
        id: 99,
        image: 'ipfs://QmYNyQbwLCYvjP743Jnud1bozcFPDSXFyYNYUmfQjYs5AQ',
        name: 'Matrix Berry #99',
        additionalFields: 'thisWontBreakValidation',
      },
      version: 1,
    },
    standard: 'nft',
    response: {
      version: 'CIP68v1',
      metadata: {
        description: '',
        id: 99,
        image: 'ipfs://QmYNyQbwLCYvjP743Jnud1bozcFPDSXFyYNYUmfQjYs5AQ',
        name: 'Matrix Berry #99',
        additionalFields: 'thisWontBreakValidation',
      },
    },
  },
  {
    name: 'Unsupported metadata version fails validation',
    payload: {
      metadata: {
        description: '',
        image: 'ipfs://QmYNyQbwLCYvjP743Jnud1bozcFPDSXFyYNYUmfQjYs5AQ',
        name: 'Matrix Berry #99',
      },
      version: 2,
    },
    standard: 'nft',
    response: false,
  },
  {
    name: 'non-number version fails validation',
    payload: {
      metadata: {
        description: '',
        image: 'ipfs://QmYNyQbwLCYvjP743Jnud1bozcFPDSXFyYNYUmfQjYs5AQ',
        name: 'Matrix Berry #99',
      },
      version: 'nan',
    },
    standard: 'nft',
    response: false,
  },
  {
    name: 'null metadata papyload',
    payload: null,
    standard: 'nft',
    response: false,
  },
  {
    name: 'FT with invalid ticker',
    payload: {
      metadata: {
        description: 'test',
        name: 'test asset',
        ticker: 58008,
        additionalFields: 'thisWontBreakValidation',
      },
      version: 1,
    },
    standard: 'ft',
    response: false,
  },
  {
    name: 'invalid standard',
    payload: {
      metadata: {
        description: '',
        id: 99,
        image: 'ipfs://QmYNyQbwLCYvjP743Jnud1bozcFPDSXFyYNYUmfQjYs5AQ',
        name: 'Matrix Berry #99',
        additionalFields: 'thisWontBreakValidation',
      },
      version: 1,
    },
    standard: 'not-valid',
    response: false,
  },
  {
    name: 'Bison Coin (invalid NFT due to missing image prop)',
    payload: {
      metadata: {
        name: 'Bison Coin',
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
      },
      version: 1,
    },
    standard: 'nft',
    response: false,
  },
  {
    name: 'Bison Coin (valid FT)',
    payload: {
      metadata: {
        name: 'Bison Coin',
        description: 'desc',
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
      },
      version: 1,
    },
    standard: 'ft',
    response: {
      version: 'CIP68v1',
      metadata: {
        name: 'Bison Coin',
        description: 'desc',
        files: [
          {
            src: 'ipfs://QmPk6SY8P4yWekK1Z9BSrLfQ8bPDHZiirWVgi5hdsyvnvd',
            name: 'BISON',
            mediaType: 'image/png',
          },
        ],
      },
    },
  },
];
