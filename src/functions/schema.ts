import fs from 'fs';
import path from 'path';
import YAML from 'yaml';
import Ajv from 'ajv';

import nutlinkAddressTickers from '../custom-schemas/nutlink-address-tickers';
import nutlinkTicker from '../custom-schemas/nutlink-ticker';
import scriptsJsonSchema from '../custom-schemas/scripts-json';
import txsMetadata from '../custom-schemas/txs-metadata';

const ajv = new Ajv({ strict: false });
const file = fs.readFileSync(
  path.resolve(__dirname, '../../openapi.yaml'),
  'utf8',
);
const spec = YAML.parse(file);

export const transformSchemaElement = (schema: any) => {
  // To generate response schema supported by fast-json-stringify
  // We need to convert array type (["null", "<other type>"]) to type: "<other type>" with nullable set to true.
  // Note: Alternative approach for values with multiple types is to use anyOf/oneOf.
  // https://github.com/fastify/fast-json-stringify#anyof-and-oneof

  if (schema.type === 'object' && schema.properties) {
    // convert type in object properties
    for (const propertyKey of Object.keys(schema.properties)) {
      const property = schema.properties[propertyKey];
      if (
        property.type === 'object' &&
        property.additionalProperties === true &&
        !property.properties
      ) {
        // Workaround for fast-json-stringify
        // If object's property is arbitrary object,
        // convert {type: 'object', additionalProperties: true} to {}
        delete schema.properties[propertyKey].type;
        delete schema.properties[propertyKey].additionalProperties;
      }
      schema.properties[propertyKey] = transformSchemaElement(
        schema.properties[propertyKey],
      );
    }
    return schema;
  } else if (schema.type === 'array' && schema.items) {
    // convert type in array items
    schema.items = transformSchemaElement(schema.items);
    return schema;
  } else if (Array.isArray(schema.type)) {
    const isNullable = schema.type.includes('null');
    if (isNullable) {
      if (schema.type.length > 2) {
        throw Error(
          `Error in ${JSON.stringify(
            schema,
          )}. Type doesn't support an array with multiple values. Use anyOf/oneOf.`,
        );
      }
      return {
        ...schema,
        type: schema.type.filter((a: string) => a !== 'null')[0],
        nullable: true,
      };
    } else {
      // edge case where type is an array with only 1 element
      if (schema.type.length === 1) {
        return {
          ...schema,
          type: schema.type[0],
        };
      }
      throw Error(
        `Error in ${JSON.stringify(
          schema,
        )}. Type doesn't support an array with multiple values. Use anyOf/oneOf.`,
      );
    }
  } else {
    // do nothing
    return schema;
  }
};

export const getSchemaForEndpoint = (endpointName: string) => {
  if (!spec.paths[endpointName]) {
    throw Error(
      `Missing Blockfrost OpenAPI schema for endpoint "${endpointName}".`,
    );
  }

  const responses: any = { response: {} };

  // Hacky way to support POST endpoints.
  // Primarily pick GET with a fallback to POST
  // TODO: return body of POST endpoints
  // https://www.fastify.io/docs/latest/Reference/Validation-and-Serialization/#validation
  const method = 'post' in spec.paths[endpointName] ? 'post' : 'get';
  const endpoint = spec.paths[endpointName][method];

  for (const response of Object.keys(endpoint.responses)) {
    // success 200
    if (response === '200') {
      const contentType =
        'application/octet-stream' in endpoint.responses['200'].content
          ? 'application/octet-stream'
          : 'application/json';
      const referenceOrValue =
        endpoint.responses['200'].content[contentType].schema;

      // is reference -> resolve references
      if (referenceOrValue['$ref']) {
        const schemaName = referenceOrValue['$ref'].replace(
          '#/components/schemas/',
          '',
        );
        const schemaReferenceOrValue = spec.components.schemas[schemaName];

        // is nested reference
        if (
          schemaReferenceOrValue.items &&
          schemaReferenceOrValue.items['$ref']
        ) {
          const nestedSchemaName = schemaReferenceOrValue.items['$ref'].replace(
            '#/components/schemas/',
            '',
          );

          if (schemaReferenceOrValue.type) {
            responses.response[200] = transformSchemaElement({
              ...schemaReferenceOrValue,
              items: spec.components.schemas[nestedSchemaName],
            });
          } else {
            responses.response[200] = transformSchemaElement(
              spec.components.schemas[nestedSchemaName],
            );
          }
        } else {
          // is not nested reference
          responses.response[200] = transformSchemaElement(
            spec.components.schemas[schemaName],
          );
        }
      } else {
        // is not reference
        responses.response[200] = transformSchemaElement(referenceOrValue);
      }

      // anyOf case
      if (referenceOrValue['anyOf']) {
        const anyOfResult: any = { anyOf: [] };

        for (const anyOfItem of referenceOrValue['anyOf']) {
          const schemaName = anyOfItem['$ref'].replace(
            '#/components/schemas/',
            '',
          );

          const item = transformSchemaElement(
            spec.components.schemas[schemaName],
          );
          anyOfResult['anyOf'].push(item);
        }

        responses.response[200] = anyOfResult;
      }

      const parameters = endpoint.parameters;

      if (parameters) {
        const queryParams = parameters.filter((i: any) => i.in === 'query');
        let queryProps: any = {};

        if (queryParams && queryParams.length > 0) {
          queryParams.forEach((param: any) => {
            delete param.schema.format;
            queryProps[param.name] = param.schema;
          });

          responses['querystring'] = {
            type: 'object',
            properties: queryProps,
          };
        }

        const pathParams = parameters.filter((i: any) => i.in === 'path');

        if (pathParams && pathParams.length > 0) {
          let pathProps: any = {};

          pathParams.forEach((param: any) => {
            delete param.schema.format;
            pathProps[param.name] = param.schema;
          });

          responses['params'] = {
            type: 'object',
            properties: pathProps,
          };
        }

        // const query = parameters.filter((i: any) => i.in === 'param');
        // let queryParams: any = {};
      }

      // custom schemas
      if (endpointName === '/txs/{hash}/metadata') {
        responses.response[200] = txsMetadata;
      }

      if (endpointName === '/nutlink/{address}/tickers/{ticker}') {
        responses.response[200] = nutlinkAddressTickers;
      }

      if (endpointName === '/nutlink/tickers/{ticker}') {
        responses.response[200] = nutlinkTicker;
      }

      if (endpointName === '/scripts/{script_hash}/json') {
        // TODO: no longer necessary
        responses.response[200] = scriptsJsonSchema;
      }
    }

    // errors and others
    else {
      responses.response[response] =
        spec.components.responses[response].content['application/json'].schema;
    }
  }

  // debug
  // if (endpointName === '/blocks/{hash_or_number}') {
  //   console.log(endpointName, JSON.stringify(responses));
  // }

  return responses;
};

export const generateSchemas = () => {
  // Returns fast-json-stringify compatible schema object indexed by endpoint name
  const endpoints = Object.keys(spec.paths);

  const schemas: Record<string, unknown> = {};
  for (const endpoint of endpoints) {
    try {
      schemas[endpoint] = getSchemaForEndpoint(endpoint);
    } catch (error) {
      console.error(`Error while processing endpoint ${endpoint}`);
      throw error;
    }
  }
  return schemas;
};

export const getSchema = (schemaName: string) => {
  if (!spec.components.schemas[schemaName]) {
    throw Error(`Missing Blockfrost OpenAPI schema with name "${schemaName}".`);
  }

  return spec.components.schemas[schemaName];
};

export const validateSchema = (schemaName: string, input: unknown) => {
  const schema = getSchema(schemaName);
  const validate = ajv.compile(schema);
  const isValid = validate(input);

  return { isValid, errors: validate.errors };
};
