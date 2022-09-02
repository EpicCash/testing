Feature: There is a set of API methods for the epic-server

Background: Defining settings
  Given The epic-server binary is at /home/raul/Documentos/Brick_Abode/v3/epic/target/release/epic
  And I am using the mainnet network

Scenario: Test get_blocks for a pruned range
  Given I started the node
  And The JSON query is for get_blocks ranging from "1" to "10"
  When I make the HTTP POST request  
  Then I got an empty set as response

Scenario: Test get_blocks for the last 10 blocks
  Given I started the node
  And The JSON query is for get_blocks ranging from "current" to "current - 10"
  When I make the HTTP POST request  
  Then I got a set with 10 blocks data

Scenario: Test get_last_n_kernels for the last 10 kernels
  Given I started the node
  And The JSON query is for get_last_n_kernels with parameter defined as 10
  When I make the HTTP POST request  
  Then I got a set with 10 or more kernels data