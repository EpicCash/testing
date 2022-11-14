Feature: Verify the longevity of a wallet, checking information in the chain referring to my wallet

  Background: Defining settings
    Given The "epic-server" binary is at "/home/jualns/Desktop/epic/target/release/epic"
    And The "epic-wallet" binary is at "/home/jualns/Desktop/epic-wallet/target/release/epic-wallet"
    And The "epic-miner" binary is at "/home/jualns/Desktop/epic-miner/target/debug/epic-miner"
    And I am using the "usernet" network

    @serial
    Scenario: Test if wallet change itself to new DB 
      Given I have a wallet in LMDB
      Then I run info command
      And I check if wallet change to new DB

    #@serial
    #Scenario: Testing the operation of a new wallet
    #  Given I initiate a wallet
    #  And I initiate a miner
    #  And I mine 11 blocks and stop miner
    #  And I have a wallet with coins
    #  When I send 0.001 coins with http method
    #  And I send 3 coins with http method
    #  Given I initiate a miner
    #  And I mine 15 blocks and stop miner
    #  When I make a recovery
    #  Then I have a wallet with 2 outputs transactions and 26 mined blocks