Feature: Test the chain synchronization and mining processes

Scenario: Test mining on usernet with onlyrandomx policy
  Given I had defined the policy as <onlyrandomx>
  And I am using the <usernet> network
  And I initiate a wallet <w1> for <usernet>
  And Wallet <w1> is at listening mode on <usernet>
  When I start the local node
  And I start the local miner
  Then After <5> minutes I can see some mined blocks
  Then the wallet <w1> have a balance more than <1>

Scenario: Test mining on usernet with noprogpow policy
  Given I had defined the policy as <noprogpow>
  And I am using the <usernet> network
  And I initiate a wallet <w1> for <usernet>
  And Wallet <w1> is at listening mode on <usernet>
  When I start the local node
  And I start the local miner
  Then After <5> minutes I can see some mined blocks
  Then the wallet <w1> have a balance more than <1>

Scenario: Test mining on floonet with onlyrandomx policy
  Given I had defined the policy as <onlyrandomx>
  And I am using the <floonet> network
  And I initiate a wallet <w1> for <floonet>
  And Wallet <w1> is at listening mode on <floonet>
  When I start the local node
  And I start the local miner
  Then After <5> minutes I can see some mined blocks
  Then the wallet <w1> have a balance more than <1>

Scenario: Test mining on floonet with noprogpow policy
  Given I had defined the policy as <noprogpow>
  And I am using the <floonet> network
  And I initiate a wallet <w1> for <floonet>
  And Wallet <w1> is at listening mode on <floonet>
  When I start the local node
  And I start the local miner
  Then After <5> minutes I can see some mined blocks
  Then the wallet <w1> have a balance more than <1>