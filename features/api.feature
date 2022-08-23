Feature: There is a set of API methods for the epic-server

Scenario: Test get_blocks for a pruned range
  Given I am using the <mainnet> network
  And The chain is synced
  And The JSON query is for <get_blocks> ranging from <1> to <10>
  When Make the HTTP POST request
  Then I got an empty set as response

Scenario: Test get_blocks for the last 10 blocks
  Given I am using the <mainnet> network
  And The chain is synced
  And The JSON query is for <get_blocks> ranging from <current chain height> to <current chain height - 10>
  When Make the HTTP POST request
  Then I got a set with 10 blocks data

Scenario: Test get_last_n_kernels for the last 10 kernels
  Given I am using the <mainnet> network
  And The chain is synced
  And The JSON query is for <get_last_n_kernels> with parameter defined as <10>
  When Make the HTTP POST request
  Then I got a set with 10 kernels data