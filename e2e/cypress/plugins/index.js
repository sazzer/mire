const cucumber = require("cypress-cucumber-preprocessor").default;
const database = require("./database");

/**
 * @type {Cypress.PluginConfig}
 */
module.exports = (on, config) => {
  database.openPool(config.env.POSTGRES_URL);
  on("task", {
    "db:reset": database.reset,
    "db:seed": database.seed,
  });
  on("file:preprocessor", cucumber());
};
