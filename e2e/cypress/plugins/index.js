const cucumber = require("cypress-cucumber-preprocessor").default;

/**
 * @type {Cypress.PluginConfig}
 */
module.exports = (on) => {
  on("file:preprocessor", cucumber());
};
