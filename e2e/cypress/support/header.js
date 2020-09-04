function getUserMenuElement() {
  return cy.findByRole("menu", { selector: "li" });
}

function getUserMenu(callback) {
  getUserMenuElement().within(($menu) => {
    if (callback) {
      callback({
        getElement: () => $menu,
        getDropdownButton: () => cy.get('a[data-toggle="dropdown"]'),
        logout: () => {
          cy.get('a[data-toggle="dropdown"]').click();
          cy.findByText("Log out", { role: "menuitem" }).click();
        },
      });
    }
  });
}

Cypress.Commands.add("getPageHeader", (callback) => {
  cy.findByRole("banner", { selector: "header" }).within(() => {
    callback({
      getUserMenu,
      getUserMenuElement,
    });
  });
});
