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

    @serial
    Scenario: Test chain synchronization on floonet
      When I start the node with policy "onlyrandomx"
      Then The chain is downloaded and synced
      And I kill all running epic systems

    @serial
    Scenario: Test connection with other peers on floonet
      When I start the node with policy "onlyrandomx"
      Then I am able to see more than one peer connected
      And I kill all running epic systems

    @serial
    Scenario: Test mining on floonet
      When I start the node with policy "onlyrandomx"
      Then The chain is downloaded and synced
      Given I know the initial height of chain
      When I start the wallet
      And I start the miner
      Given I mine some blocks into my wallet
      When I stop the miner
      Then The chain_height from peers is more than initial value
      And I kill all running epic systems