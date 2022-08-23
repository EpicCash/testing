Feature: Start a node on usernet

Scenario: Test if the node can start at usernet with default policy
  Given I have the <default> policy
  And I am at <usernet>
  When I start the node
  Then The node is running




# args:
#   - floonet:
#       help: Run epic against the Floonet (as opposed to mainnet)
#       long: floonet
#       takes_value: false
#   - usernet:
#       help: Run epic as a local-only network. Doesn't block peer connections but will not connect to any peer or seed
#       long: usernet
#       takes_value: false
#   - noprogpow:
#       help: Run epic floonet or usernet without progpow blocks
#       long: noprogpow
#       takes_value: false
#   - onlyrandomx:
#       help: Run epic floonet or usernet only with randomx blocks
#       long: onlyrandomx
#       takes_value: false