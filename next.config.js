module.exports = {
  async redirects() {
    return [
      {
        source:
          '#tag/Cardano-Accounts/paths/~1accounts~1%7Bstake_address%7D/get',
        destination: 'https:/blockfrost.dev/api/specific-account-address',
        permanent: true,
      },
    ];
  },
};
