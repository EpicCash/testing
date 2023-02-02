Feature: Test the methods of transactions and interactions between send, receive and finalize

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    When I start the miner
    Given I mine some blocks into my wallet

  @serial
  Scenario: Test Emoji send methods with 0.0001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.001 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Emoji send methods with 14 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 14 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Emoji send methods with 0.00000001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Emoji send methods with 2 transactions
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    When I send 14 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    And I await confirm the transaction
    Then I have 4 new transactions in txs
    And I kill all running epic systems
