Feature: Test the operations that the chain does on itself, test the cut-through

Feature: There is a need to validate the floonet environment

  #"/home/jualns/Desktop/epic/target/release/epic"
  #"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  #"/home/ba/Desktop/epic-miner/target/debug/epic-miner"

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network

  Scenario:  Testing the cut-through on wallet transactions
    Given I have a <testing> chain
    And I have a wallet with <4> coins
    When I receive <x> coins from another wallet #see if this step is needed
    And I send <x/2> coins with <HTTP> method
    And I await the confirm transaction
    Then I have a wallet with <2> outputs transactions
