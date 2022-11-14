Feature: Test the methods of transactions and interactions between send, receive and finalize

  #"/home/ba/Desktop/EpicV3/epic/target/release/epic"
  #"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  #"/home/ba/Desktop/epic-miner/target/debug/epic-miner"

    Background: Defining settings
      Given The "epic-server" binary is at "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic\\target\\release\\epic.exe" 
      And The "epic-wallet" binary is at "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic-wallet\\target\\release\\epic-wallet.exe" 
      And The "epic-miner" binary is at "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic-miner\\epic-miner.exe" 
      And I am using the "usernet" network
      And I mine some blocks into my wallet
  
      @serial
      Scenario: Test Self send methods
        Given I have a wallet with coins
        When I send 0.001 coins with self method
        And I await confirm the transaction
        Then I have 2 new transactions in txs
        And I kill all running epic systems

      @serial
      Scenario: Test HTTP send methods
        Given I have a wallet with coins
        When I send 0.001 coins with http method
        And I await confirm the transaction
        Then I have 2 new transactions in txs
        And I kill all running epic systems
      
      @serial
      Scenario: Test Emoji send methods
        Given I have a wallet with coins
        When I send 0.001 coins with emoji method
        And I receive the emoji transaction
        And I finalize the emoji transaction
        And I await confirm the transaction
        Then I have 2 new transactions in txs
        And I kill all running epic systems

      @serial
      Scenario: Test File send methods
        Given I have a wallet with coins
        When I send 0.001 coins with file method
        And I receive the file transaction
        And I finalize the file transaction
        And I await confirm the transaction
        Then I have 2 new transactions in txs
        And I kill all running epic systems

      #Scenario:  Test Keybase/TOR send methods
      #  Given I have a testing chain
      #  And I have a wallet with <2> coins
      #  When I send <1e-5> coins with <Keybase> method
      #  And I await the confirm transaction
      #  Then I have 2 new transactions in outputs with <1e-5> coins