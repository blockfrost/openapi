module.exports = {
  async redirects() {
    return [
      // root
      {
        source: '/',
        destination:
          'http://www.blockfrost.dev/api/blockfrost-io-api-documentation',
        permanent: true,
      },
      // addresses
      {
        source:
          '/#tag/Cardano-Accounts/paths/~1accounts~1%7Bstake_address%7D/get',
        destination: 'http://www.blockfrost.dev/api/specific-account-address',
        permanent: true,
      },
    ];
  },
};
