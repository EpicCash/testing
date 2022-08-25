Feature: There is a set of API methods for the epic-server

Background: Defining settings
  Given The epic-server binary is at /home/raul/Documentos/Brick_Abode/v3/epic/target/release/epic
  And I am using the mainnet network
  And The chain is synced

Scenario: Test get_blocks for a pruned range
  Given The JSON query is for <get_blocks> ranging from <1> to <10>
  When Make the HTTP POST request
  Then I got an empty set as response

# Scenario: Test get_blocks for the last 10 blocks
#   Given The JSON query is for <get_blocks> ranging from <current chain height> to <current chain height - 10>
#   When Make the HTTP POST request
#   Then I got a set with 10 blocks data

# Scenario: Test get_last_n_kernels for the last 10 kernels
#   Given The JSON query is for <get_last_n_kernels> with parameter defined as <10>
#   When Make the HTTP POST request
#   Then I got a set with 10 kernels data