# Core Domain

## Core Entities

### Node
The fundamental building block of the conversation flow.

Base Node Properties:
- id: Unique identifier
- type: Type of node (abstract, can be extended)
- name: Human readable name
- description: Node purpose and functionality
- inputs: Expected input parameters
- outputs: Expected output parameters
- conditions: Entry conditions for the node
- actions: Operations to perform when node is active

Common Node Extensions:
- MessageNode: Displays information to user
- InputNode: Captures user input
- DecisionNode: Handles branching logic
- IntegrationNode: Connects with external systems
- HandoffNode: Transfers to human agents

### Edge
Represents connections between nodes, defining the possible paths in the flow.

Properties:
- id: Unique identifier
- sourceNodeId: Origin node
- targetNodeId: Destination node
- conditions: Transition conditions
- metadata: Additional routing information

### ConversationState
Temporary state maintained during a single user interaction session.

Properties:
- conversationId: Unique session identifier
- currentNodeId: Current active node
- userInputs: History of user inputs in current session
- contextVariables: Session-scoped variables
- timestamp: Last interaction time
- metadata: Additional session data

### FlowState
Persistent state available across all nodes and conversations.

Properties:
- flowId: Unique flow identifier
- globalVariables: Flow-wide variables
- configurations: Flow-level settings
- statistics: Flow performance metrics
- version: Flow version information

### StateNavigator
Manages conversation progression through nodes.

Properties:
- currentState: Current conversation state
- nextNodeId: Next node to process
- transitionHistory: Path taken through nodes
- navigationRules: Rules for determining next node
- fallbackStrategy: Handling for invalid transitions

## Core Behaviors

1. Node Processing:
   - Node activation/deactivation
   - Input validation
   - Action execution
   - Output generation

2. State Management:
   - State initialization
   - State updates
   - State persistence
   - State recovery

3. Flow Navigation:
   - Node transition validation
   - Path determination
   - Edge traversal
   - Error handling

4. Context Management:
   - Variable scoping
   - Context isolation
   - Data persistence
   - Memory management


Theres going to be 2 types of nodes: 
1. Conversational nodes
2. Non-conversational nodes
   - These are nodes that are not part of the conversation flow, but are still part of the application logic
   - This are going to implement ConversationalBehavior.
   - Examples:
     - System messages
     - Integration with external systems
     - User-initiated actions
     - Data processing
     - Data storage