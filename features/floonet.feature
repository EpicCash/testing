Feature: There is a need to validate the floonet environment

	Background: Defining settings
		Given Define "epic-server" binary
		And Define "epic-wallet" binary
		And Define "epic-miner" binary
		And I am using the "floonet" network
	#And I mine some blocks into my wallet

	@serial
	Scenario: Test mining on floonet
		When I start the node with policy "onlyrandomx"
		Then The chain is downloaded and synced
		Given I know the initial height of chain
		When I start the wallet
		And I start the miner
		Given I mine some blocks into my wallet
		When I stop the miner
		Then The chain_height from peers is more than initial value
		And I kill all running epic systems

	@serial
	Scenario: Test chain synchronization on floonet
		When I start the node with policy "onlyrandomx"
		Then The chain is downloaded and synced
		And I kill all running epic systems

	@serial
	Scenario: Test connection with other peers on floonet
		When I start the node with policy "onlyrandomx"
		Then I am able to see more than one peer connected
		And I kill all running epic systems
