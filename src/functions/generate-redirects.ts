const structure = [
  /* Your provided structure */
];

const baseUrl = 'https://blockfrost.dev/api';

const redirects = structure.flatMap(entry => {
  if (entry.type === 'doc') {
    // Handle 'doc' type
    return {
      source: `/${entry.id}`,
      destination: `${baseUrl}/${entry.id}`,
      permanent: true,
    };
  } else if (entry.type === 'category') {
    // Handle 'category' type
    return entry.items.map(item => {
      return {
        source: `/${item.id}`,
        destination: `${baseUrl}/${item.id}`,
        permanent: true,
      };
    });
  }
  return [];
});

const redirectConfig = {
  redirects,
};

console.log(JSON.stringify(redirectConfig, null, 2));
