Feature: Build a wallet with high nimber of transactions

  #"/home/ba/Desktop/EpicV3/epic/target/release/epic"
  #"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  #"/home/ba/Desktop/epic-miner/target/debug/epic-miner"
  
  # Run before cucumber start, before all steps
  # Need run before because we want stop the systems after all scenarios finish
  #   Given The "epic-server" binary is at "/home/jualns/Desktop/epic/target/release/epic" 
  #   Given The "epic-wallet" binary is at "/home/jualns/Desktop/epic-wallet/target/release/epic-wallet" 
  #   Given The "epic-miner" binary is at "/home/jualns/Desktop/epic-miner/targe/debug/epic-miner" 
  #   Given I am using the "usernet" network
  #
  # Run after cucumber finish
  #   When I await confirm the transactions
  #   Then I kill all running epic systems

    Background: Defining settings
      Given I mine some blocks into my wallet
      And I want use http method
      And I want send 2 transactions

      Scenario: Make a lot of transactions 1/2
        Given I have a wallet with coins
        When I send 1 coins with http method

      Scenario: Make a lot of transactions 2/2
        Given I have a wallet with coins
        When I send 2 coins with http method