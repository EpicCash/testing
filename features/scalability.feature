Feature: Test longevity and stress the systems

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    Given I am using the "usernet" network

  @serial
  Scenario: Testing the operation of a huge wallet - http
    # "new","stored-tiny", "stored-huge", "passphrase-tiny", "passphrase-huge"
    Given I use a "stored-huge" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    # Test time
    When I make a 50 transactions with http method
    Then The average transaction time is less than 15 second
    # Test confirm transactions and mine
    When I start the miner
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallet
    Then I run scan
    # Save all informations in `info`, `txs` and `outputs`
    Then I run and save info command
    Then I run and save txs command
    Then I run and save outputs command
    When I delete the wallet folder
    # Test recover
    When I make the recover in my wallet
    Then I have the same information
    Then I have the same outputs
    Then I have the same transactions
    When I stop the node

  @serial
  Scenario: Testing the operation of a huge wallet - self
    # "new","stored-tiny", "stored-huge", "passphrase-tiny", "passphrase-huge"
    Given I use a "stored-huge" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    # Test time
    When I make a 5 transactions with self method
    Then The average transaction time is less than 10 second
    # Test confirm transactions and mine
    When I start the miner
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallethuge
    Then I have the same information
    Then I have the same outputs
    Then I have the same transactions
    When I stop the node

  @serial
  Scenario: Testing the operation of a tiny wallet - http
    # "new","stored-tiny", "stored-huge", "passphrase-tiny", "passphrase-huge"
    Given I use a "stored-tiny" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    # Test time
    When I make a 50 transactions with http method
    Then The average transaction time is less than 15 second
    # Test confirm transactions and mine
    When I start the miner
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallet
    Then I run scan
    # Save all informations in `info`, `txs` and `outputs`
    Then I run and save info command
    Then I run and save txs command
    Then I run and save outputs command
    When I delete the wallet folder
    # Test recover
    When I make the recover in my wallet
    Then I have the same information
    Then I have the same outputs
    Then I have the same transactions
    When I stop the node

  @serial
  Scenario: Testing the operation of a tiny wallet - self
    # "new","stored-tiny", "stored-huge", "passphrase-tiny", "passphrase-huge"
    Given I use a "stored-tiny" wallet
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    # Test time
    When I make a 5 transactions with self method
    Then The average transaction time is less than 10 second
    # Test confirm transactions and mine
    When I start the miner
    Then I await confirm the transaction
    When I stop the miner
    And I stop the wallet
    Then I run scan
    # Save all informations in `info`, `txs` and `outputs`
    Then I run and save info command
    Then I run and save txs command
    Then I run and save outputs command
    When I delete the wallet folder
    # Test recover
    When I make the recover in my wallet
    Then I have the same information
    Then I have the same outputs
    Then I have the same transactions
    When I stop the node

  @serial
  Scenario: Test transaction time 1
    When I start the wallet
    When I start the miner
    When I mine some blocks into my wallet
    When I make a 20 transactions with http method
    Then The average transaction time is less than 0.75 second
    And I await confirm the transaction
    And All transactions work
    And I kill all running epic systems

  @serial
  Scenario: Test transaction time 2
    When I start the wallet
    When I start the miner
    When I mine some blocks into my wallet
    When I make a 100 transactions with self method
    Then The average transaction time is less than 0.75 second
    And I await confirm the transaction
    And All transactions work
    And I kill all running epic systems
# Scenario planned but not yet done
#@serial
#Scenario: Connection multiple nodes
#  Given I am using the <floonet> network
#  When I create a new HOME and start a new node <10> times
#  Then The nodes connect to another
