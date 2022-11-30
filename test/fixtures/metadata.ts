export const parseOnChainMetadataFixtures = [
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
        add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5: {
          '8424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751': {
            name: 'Optim EQT',
            image: [
              'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
              'di',
            ],
          },
        },
        ec4358d0daae8ab25facf91eff42ad60175476d620f6c22193176e02: {
          '8424fcf2617ba79f8089f860c2ce679d14345c9b153d0c14ea0481eaa0624751': {
            name: 'Optim Borrower Token',
            image: [
              'ipfs://bafkreids5uegktmjz753qngxmjlttzrx7dg5qeqqsnf734csuijpkg24',
              'ne',
            ],
          },
        },
      },
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
    name: 'valid on-chain metadata version 2 with asset_name as empty string',
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
        add8604a36a46446dd22281473614c5b390afbc064ff1338516b19f5: {
          '': {
            name: 'Optim EQT',
            image: [
              'ipfs://bafkreif5iapksurpzoegyxl7jybdlxbqsz2upsagu2dmbygj4qbf6cfc',
              'di',
            ],
          },
        },
      },
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
    name: 'invalid on-chain metadata version 2 with asset_name in UTF-8',
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
