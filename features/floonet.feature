Feature: There is a need to validate the floonet environment

  #"/home/jualns/Desktop/epic/target/release/epic"
  #"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  #"/home/ba/Desktop/epic-miner/target/debug/epic-miner"

  Background: Defining settings
        Given The "epic-server" binary is at "/home/jualns/Desktop/epic/target/release/epic"
        And The "epic-wallet" binary is at "/home/jualns/Desktop/epic-wallet/target/release/epic-wallet"
        And The "epic-miner" binary is at "/home/jualns/Desktop/epic-miner/target/release/epic-miner"
        And I am using the "floonet" network
        #And I mine some blocks into my wallet

    #@serial
    #Scenario: Test chain synchronization on floonet
    #  When I start the node with policy "onlyrandomx"
    #  Then The chain is downloaded and synced
    #  And I kill all running epic systems

    #@serial
    #Scenario: Test connection with other peers on floonet
    #  When I start the node with policy "onlyrandomx"
    #  Then I am able to see more than one peer connected
    #  And I kill all running epic systems

    @serial
    Scenario: Test mining on floonet
      Given I copy the unsynced chain
      When I start the node with policy "noprogpow"
      Given I create the wallet
      And I start the miner
      Given I mine some blocks into my wallet
      Then I stop the miner
      When I start the local node
      And I start the local miner
      Then After <5> minutes I can see some mined blocks
      Then the wallet <w1> have a balance more than <1>