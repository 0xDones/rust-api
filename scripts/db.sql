CREATE DATABASE drop_cases;
CREATE ROLE dev_api WITH LOGIN PASSWORD 'drop-cases-dev-pwd';
GRANT ALL PRIVILEGES ON database drop_cases to drop_cases;


