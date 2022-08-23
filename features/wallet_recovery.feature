Feature: Verify the longevity of a wallet, checking information in the chain referring to my wallet

Scenario:  Testing the operation of a new wallet
  Given I have a <testing> chain
  And I initiate a wallet <w1>
  And I mine <7> blocks
  When I send <1> coins with <HTTP> method
  Given I add <5> blocks on chain mined with <randomx> and accept
  When I receive <2> coins from another wallet #see if this step is needed
  Given I add <7> blocks on chain mined with <cuckoo> and accept
  When I make a recovery
  Then I have a wallet with <2> outputs transactions and <7> mined blocks