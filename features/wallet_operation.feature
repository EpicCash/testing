Feature: Verify the longevity of a wallet, checking information in the chain referring to my wallet

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network
    When I start the node with policy "onlyrandomx"

  @serial
  Scenario: Testing the operation of a new wallet
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
  Scenario: Test if wallet change itself to new DB
    Given I have a wallet in LMDB
    Then I run info command
    And I check if wallet change to new DB
