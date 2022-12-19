Feature: This feature will check the migration process from LMDB database to SQLite and test its functionalities

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network
  #And I mine some blocks into my wallet

  @serial
  Scenario: Test if wallet change itself to new DB
    Given I start the node
    Then I have an LMDB based wallet
    When I run info command
    Then I have an SQLite based wallet