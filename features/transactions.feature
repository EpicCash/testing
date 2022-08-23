Feature: Test the methods of transactions and interactions between send, receive and finalize

Scenario:  Test File send methods
  Given I have a <testing> chain
  And I have a wallet with <2> coins
  When I send <1e-5> coins with <File> method
  And I receive the <File> transaction response
  And I finalize the <File> transaction
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with <1e-5> coins

Scenario:  Test Emoji send methods
  Given I have a <testing> chain
  And I have a wallet with <2> coins
  When I send <1e-5> coins with <Emoji> method
  And I receive the <Emoji> transaction response
  And I finalize the <Emoji> transaction
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with <1e-5> coins

Scenario:  Test HTTP send methods
  Given I have a <testing> chain
  And I have a wallet with <2> coins
  When I send <1e-5> coins with <HTTP> method
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with <1e-5> coins

Scenario:  Test Keybase/TOR send methods
  Given I have a <testing> chain
  And I have a wallet with <2> coins
  When I send <1e-5> coins with <Keybase> method
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with <1e-5> coins

Scenario:  Test Self send methods
  Given I have a <testing> chain
  And I have a wallet with <2> coins
  When I send <1e-5> coins with <Send> method
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with <1e-5> coins