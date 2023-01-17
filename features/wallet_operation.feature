Feature: Verify the longevity of a wallet, checking information in the chain referring to my wallet

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network

  @serial
  Scenario: Testing the operation of a new wallet - 1
    Given I use a "new" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    And I start the miner
    # 19 >= 0.001 + 3 + 15
    And I mine 19 coins into my wallet
    # Test a float value < 1.
    When I send 0.001 coins with http method
    # Test an amount smaller than a block, < approximately 14.52 coins.
    And I send 3 coins with http method
    # Test a value greater than one block, to use more than 1 output to create a new transaction.
    And I send 15 coins with http method
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallet
    Then I run and save info command
    When I delete the wallet folder
    When I make the recover in my wallet
    Then I have the same information
    When I stop the node

  @serial
  Scenario: Testing the operation of a new wallet - 2
    Given I use a "new" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    And I start the miner
    # 60 >= 15.0000001 + 14 + 30
    And I mine 60 coins into my wallet
    # Test a float value < 1.
    When I send 15.0000001 coins with self method
    # Test an amount smaller than a block, < approximately 14.52 coins.
    And I send 14 coins with self method
    # Test a value greater than one block, to use more than 1 output to create a new transaction.
    And I send 30 coins with self method
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallet
    Then I run and save info command
    When I delete the wallet folder
    When I make the recover in my wallet
    Then I have the same information
    When I stop the node

  @serial
  Scenario: Test if wallet change itself to new DB - tiny
    Given I use a "stored-tiny" wallet
    When I start the node with policy "onlyrandomx"
    Given I have a wallet in LMDB
    Then I run and save info command
    And I check if wallet change to new DB

  @serial
  Scenario: Test if wallet change itself to new DB - huge
    Given I use a "stored-huge" wallet
    When I start the node with policy "onlyrandomx"
    Given I have a wallet in LMDB
    Then I run and save info command
    And I check if wallet change to new DB
