Feature: Authentication

  Scenario: List the available authentication providers
    Then the available authentication providers are:
      | Google |

  Scenario Outline: Authenticating as a new user: <Provider>
    When I authenticate using "<Provider>"
    Then I am logged in as "<Name>"

    Examples:
      | Provider | Name      |
      | Google   | Test User |