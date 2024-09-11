export const error400 = {
  properties: {
    error: {
      example: 'Bad Request',
      type: 'string',
    },
    message: {
      example: 'Backend did not understand your request.',
      type: 'string',
    },
    status_code: {
      example: 400,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};

export const error403 = {
  properties: {
    error: {
      example: 'Forbidden',
      type: 'string',
    },
    message: {
      example: 'Invalid project token.',
      type: 'string',
    },
    status_code: {
      example: 403,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};

export const error418 = {
  properties: {
    error: {
      example: 'Requested Banned',
      type: 'string',
    },
    message: {
      example: 'IP has been auto-banned for flooding.',
      type: 'string',
    },
    status_code: {
      example: 418,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};

export const error429 = {
  properties: {
    error: {
      example: 'Project Over Limit',
      type: 'string',
    },
    message: {
      example: 'Usage is over limit.',
      type: 'string',
    },
    status_code: {
      example: 429,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};

export const error500 = {
  properties: {
    error: {
      example: 'Internal Server Error',
      type: 'string',
    },
    message: {
      example: 'An unexpected response was received from the backend.',
      type: 'string',
    },
    status_code: {
      example: 500,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};

export const error404 = {
  properties: {
    error: {
      example: 'Not Found',
      type: 'string',
    },
    message: {
      example: 'The requested component has not been found.',
      type: 'string',
    },
    status_code: {
      example: 404,
      type: 'integer',
    },
  },
  required: ['error', 'message', 'status_code'],
  type: 'object',
};
