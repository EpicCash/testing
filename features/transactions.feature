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
  Scenario: Test Self send methods with 0.0001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.001 coins with self method
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test HTTP send methods with 0.0001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.001 coins with http method
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

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
  Scenario: Test File send methods with 0.0001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.001 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Self send methods with 14 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 14 coins with self method
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test HTTP send methods with 14 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 14 coins with http method
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
  Scenario: Test File send methods with 14 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 14 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Self send methods with 0.00000001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with self method
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test HTTP send methods with 0.00000001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with http method
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
  Scenario: Test File send methods with 0.00000001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    And I await confirm the transaction
    Then I have 2 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test Self send methods with 2 transactions
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with self method
    When I send 14 coins with self method
    And I await confirm the transaction
    Then I have 4 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test HTTP send methods with 2 transactions
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with http method
    When I send 14 coins with http method
    And I await confirm the transaction
    Then I have 4 new transactions in txs
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

  @serial
  Scenario: Test File send methods with 2 transactions
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    When I send 14 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    And I await confirm the transaction
    Then I have 4 new transactions in txs
    And I kill all running epic systems

  @serial
  Scenario: Test All send methods with 0.00000001 coins
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with file method
    And I receive the file transaction
    And I finalize the file transaction
    When I send 0.00000001 coins with emoji method
    And I receive the emoji transaction
    And I finalize the emoji transaction
    When I send 0.00000001 coins with self method
    When I send 0.00000001 coins with http method
    And I await confirm the transaction
    Then I have 8 new transactions in txs
    And I kill all running epic systems

# Scenario planned but not yet done
#Scenario:  Test Keybase/TOR send methods
#  Given I have a testing chain
#  And I have a wallet with <2> coins
#  When I send <1e-5> coins with <Keybase> method
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins
