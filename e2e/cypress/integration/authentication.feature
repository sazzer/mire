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

  Scenario Outline: Authenticating as an existing user: <Provider>
    Given users exist with details:
      | Display Name | <Name>        |
      | Credentials  | <Credentials> |
    When I authenticate using "<Provider>"
    Then I am logged in as "<Name>"

    Examples:
      | Provider | Name          | Credentials                                |
      | Google   | Existing User | google/testuserid-1234567890/Existing User |

  Scenario: Logging out
    Given I authenticate using "Google"
    And I am logged in as "Test User"
    When I log out
    Then I am not logged in
