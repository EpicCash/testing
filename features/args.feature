Feature: Arithmetic operations

  # Let's start with addition. BTW, this is a comment.
  Scenario: User wants to multiply two numbers
    Given the numbers "2" and "3"
    When the User adds them
    Then the User gets 6 as result
    
# Feature: Start a node on usernet

# Scenario: Test if the node can start at usernet with default policy
#   Given I have the <default> policy
#   And I am at <usernet>
#   When I start the node
#   Then The node is running