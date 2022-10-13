Feature: Test the methods of transactions and interactions between send, receive and finalize

  #"/home/ba/Desktop/EpicV3/epic/target/release/epic"
  #"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  #"/home/ba/Desktop/epic-miner/target/debug/epic-miner"

    Background: Defining settings
      Given The "epic-server" binary is at "/home/jualns/Desktop/epic/target/release/epic" 
      And The "epic-wallet" binary is at "/home/jualns/Desktop/epic-wallet/target/release/epic-wallet"
      And The "epic-miner" binary is at "/home/jualns/Desktop/epic-miner/target/debug/epic-miner"
      And I am using the "floonet" network
    #  And I mine some blocks into my wallet

      @serial
      Scenario: Test HTTP send methods
        Given I have a wallet with coins
        When I send 0.001 coins with http method
        And I await confirm the transaction
        Then I have 2 new transactions in txs
        And I kill all running epic systems

      #Scenario:  Test Keybase/TOR send methods
      #  Given I have a testing chain
      #  And I have a wallet with <2> coins
      #  When I send <1e-5> coins with <Keybase> method
      #  And I await the confirm transaction
      #  Then I have 2 new transactions in outputs with <1e-5> coins

      @serial
      Scenario: Test transaction time
        When I make a 2 transactions with http method
        Then The average transaction time is less than "1" second
        And I confirm all transactions
        And All transactions work
        And I kill all running epic systems

      #@serial
      #Scenario: Connection multiple nodes
      #  Given I am using the <floonet> network
      #  When I create a new HOME and start a new node <10> times
      #  Then The nodes connect to another

      #@serial
      #Scenario: Testing the operation of a huge wallet
      #  Given I am using the <usernet> network
      #  And I start the local node
      #  And I initiate a wallet <Vitex_test> as w1
      #  And I initiate a wallet <new> as w2
      #  And Wallet <w2> is at listening mode on <usernet>
      #  When I make a <100> transactions from <w1> to <w2>
      #  And I start the miner
      #  Then I confirm all transactions
      #  Then I check <txs> informations
      #  Then I check <outputs> informations
      #  Then I check <info> informations