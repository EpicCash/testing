Feature: Test the methods of transactions and interactions between send, receive and finalize

Background: Defining settings
  Given The "epic-server" binary is at "/home/ba/Desktop/EpicV3/epic/target/release/epic"
  And The "epic-wallet" binary is at "/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet"
  And The "epic-miner" binary is at "/home/ba/Desktop/epic-miner/target/debug/epic-miner"
  And I am using the "usernet" network
  And I mine some blocks into my wallet

#Scenario:  Test File send methods
#  Given I have a testing chain
#  And I configure server and wallet toml
#  And I have a wallet with 2 coins
#  When I send <1e-5> coins with <File> method
#  And I receive the <File> transaction response
#  And I finalize the <File> transaction
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins

#Scenario:  Test Emoji send methods
#  Given I have a testing chain
#  And I have a wallet with <2> coins
#  When I send <1e-5> coins with <Emoji> method
#  And I receive the <Emoji> transaction response
#  And I finalize the <Emoji> transaction
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins

#Scenario:  Test HTTP send methods
#  Given I have a testing chain
#  And I have a wallet with <2> coins
#  When I send <1e-5> coins with <HTTP> method
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins

#Scenario:  Test Keybase/TOR send methods
#  Given I have a testing chain
#  And I have a wallet with <2> coins
#  When I send <1e-5> coins with <Keybase> method
#  And I await the confirm transaction
#  Then I have 2 new transactions in outputs with <1e-5> coins

Scenario:  Test Self send methods
  Given I have a wallet with coins
  When I send 0.001 coins with self method
  And I await the confirm transaction
  Then I have 2 new transactions in outputs with 0.001 coins sent

#Scenario: User wants to multiply two numbers
#    Given I have a chain
    #Given the numbers "2" and "3"
    #When the User multiply them
    #Then the User gets "6" as result