Feature: There is a need to validate the floonet environment

Scenario: Test chain synchronization on floonet
  Given I had defined the policy as <noprogpow>
  And I am using the <floonet> network
  When I start the local node
  Then The chain is downloaded and synced

Scenario: Test connection with other peers on floonet
  Given I had defined the policy as <noprogpow>
  And I am using the <floonet> network
  When I start the local node
  Then The chain is downloaded
  Then The chain is synced
  Then I am able to see more than one peer connected

Scenario: Test mining on floonet
  Given I had defined the policy as <noprogpow>
  And I am using the <floonet> network
  And I initiate a wallet <w1> for <floonet>
  And Wallet <w1> is at listening mode on <floonet>
  And There is more then one peer mining
  When I start the local node
  And I start the local miner
  Then After <5> minutes I can see some mined blocks
  Then the wallet <w1> have a balance more than <1>