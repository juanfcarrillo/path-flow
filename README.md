## Core Capabilities

### Entry Points & Triggers
- Define how a conversation begins (initial message, user typing, specific events).
- Handle triggers that initiate specific parts of the chatbot flow.

### User Input & Natural Language Understanding (NLU)

**User Input Processing**
- Receive raw user input (text or structured data) from external channels.
    - Text
    - Structured data
    - Images
    - Audio

**Intent Recognition**
- Use AI to analyze input and identify the user's goal or intention. This is going to be achieved through a LLM.

### Conversational Flow Management (Dialog Management)
- Acts as the "brain" of the chatbot flow.

**Branching Logic**
- Determines conversation paths based on user input, recognized intents, and extracted entities.
    - Conditional branching with intents
    - Structured branching
    - Entity-based branching

**Context and Memory**
- Maintains past interaction records to ensure contextual understanding.
    - Memory storage
        - Conversation history
    - Timeout management
    - Context management
        - Conversation state
        - Flow state

**Fallback Responses**
- Provides default guidance when input is not understood.

<!-- **Filters**
- Applies rules to tailor responses based on user data or interaction history. -->

### Bot Responses & Actions

**Messages & Information Display**
- Generates text replies and presents requested data (e.g., FAQs, product details).
    - Templated responses
    - Flow bussines context (PDFs, images, etc.)

**Asking Questions**
- Prompts users for clarification or additional information.
    - Related with intents

**Integrations/API Requests**
- Connects with external systems (e.g., CRM, databases, payment gateways) to retrieve or send data.
    - This is going to be achieved through a LLM and tooling.

### Knowledge Base Integration

- Accesses a repository of FAQs, product details, company policies, etc., for Q&A scenarios.
- It could be specific for the node or a general knowledge base.

### Exit Points & Human Handoff

**Meaningful Endings**
- Concludes conversations gracefully, confirming actions or offering further help.

**Handoff to Human Agent**
- Transfers conversation, with context, to a human agent when needed.

### Session Management
- Ensures robust session handling, unique identification, and persistence of conversation data for a defined time.

### Pluggable Architecture
- Modular design enabling developers to swap components (e.g., NLU providers, session storage, custom actions).

### Advanced Capabilities (Future/Consideration)

**Sentiment Analysis**
- Detects emotional tone to tailor responses or trigger actions (e.g., human handoff for negative sentiment).

<!-- **Multi-Language Support**
- Supports handling and responding in multiple languages. -->

<!-- **Generative AI Integration**
- Optionally integrates LLMs for more dynamic responses alongside templated ones. -->

**Analytics and Monitoring Hooks**
- Provides data logging hooks for conversation analysis, NLU performance, and user satisfaction tracking.
