import { generateSchemas } from '../index';

const jsonSchema = generateSchemas();

console.log(JSON.stringify(jsonSchema));
