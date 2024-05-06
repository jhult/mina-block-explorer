import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "dialog", () => {
  beforeEach(() => {
    cy.visit(`/summary/accounts/${DEFAULT_ACCOUNT_PK}`);
  });

  it(`has correct sections`, () => {
    ["User Commands", "SNARK Jobs", "Block Production"].forEach((section) => {
      cy.get("section h2").contains(section, {
        timeout: 60000,
      });
    });
  });
});
