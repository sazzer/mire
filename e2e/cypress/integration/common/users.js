import { Given } from "cypress-cucumber-preprocessor/steps";
import { SeedUser } from "./data/user";
import { toObjectsTall } from "./table";

Given("users exist with details:", (dataTable) => {
  toObjectsTall(dataTable)
    .map((user) => new SeedUser(user))
    .forEach(cy.seedData);
});
