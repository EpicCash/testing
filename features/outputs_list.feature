Feature: Test the operations that the chain does on itself, test the cut-through

Scenario:  Testing the cut-through on wallet transactions
  Given I have a <testing> chain
  And I have a wallet with <4> coins
  When I receive <x> coins from another wallet #see if this step is needed
  And I send <x/2> coins with <HTTP> method
  And I await the confirm transaction
  Then I have a wallet with <2> outputs transactions