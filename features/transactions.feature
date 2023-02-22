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
  Scenario: Test QR send methods with 2 transactions
    Given I have a wallet with coins
    Then I run and save txs command
    When I send 0.00000001 coins with qr method
    And I receive the qr transaction
    And I finalize the qr transaction
    When I send 0.02 coins with qr method
    And I receive the qr transaction
    And I finalize the qr transaction
    And I await confirm the transaction
    Then I have 4 new transactions in txs
    And I kill all running epic systems

# Scenario planned but not yet done
#Scenario:  Test Keybase/TOR send methods
#  Given I have a testing chain
#  And I have a wallet with <2> coins
#  When I send <1e-5> coins with <Keybase> method
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins
