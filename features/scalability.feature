Feature: Test longevity and stress the systems

  Background: Defining settings
    Given Define "epic-server" binary
    And Define "epic-wallet" binary
    And Define "epic-miner" binary
    And I am using the "usernet" network
    When I start the node with policy "onlyrandomx"
    When I start the wallet
    And I start the miner
    And I mine some blocks into my wallet

  @serial
  Scenario: Test transaction time
    When I make a 20 transactions with http method
    Then The average transaction time is less than 0.75 second
    And I await confirm the transaction
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
