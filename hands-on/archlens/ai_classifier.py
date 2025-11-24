"""
AI Classification module for ArchLens
"""

import json
from openai import OpenAI
from dotenv import load_dotenv
import os

# Load environment variables
load_dotenv()

def load_prompt(prompt_name="default"):
    """Load a prompt from prompts.json file"""
    try:
        with open('prompts.json', 'r') as f:
            prompts = json.load(f)

        prompt_config = prompts.get(prompt_name, prompts.get("default"))
        if not prompt_config:
            raise ValueError(f"Prompt '{prompt_name}' not found")

        # Format the prompt from structured JSON
        formatted = format_prompt(prompt_config)
        return formatted
    except FileNotFoundError:
        # Fallback to env var if file not found
        return os.getenv("ARCHLENS_SYSTEM_PROMPT", DEFAULT_PROMPT_FALLBACK)
    except Exception as e:
        print(f"⚠️ Error loading prompt: {e}")
        return os.getenv("ARCHLENS_SYSTEM_PROMPT", DEFAULT_PROMPT_FALLBACK)

def format_prompt(config):
    """Format structured prompt config into readable text"""
    lines = []

    lines.append(config["role"])
    lines.append("")

    lines.append("CLASSIFICATION TAXONOMY:")
    for item in config["classification_taxonomy"]:
        lines.append(f"\n{item['category']}")
        lines.append(f"  Core Intent: {item['core_intent']}")
        lines.append(f"  Signals: {item['signals']}")
        lines.append(f"  Reader Goal: {item['reader_goal']}")

    lines.append("\nTIE-BREAKING RULES:")
    for rule in config["tie_breaking_rules"]:
        lines.append(f"  - {rule}")

    lines.append("\nTAGGING RULES:")
    for rule in config["tagging_rules"]:
        lines.append(f"  - {rule}")

    lines.append(f"\n{config['output_format']}")

    return "\n".join(lines)

# Fallback prompt if prompts.json is missing
DEFAULT_PROMPT_FALLBACK = """You are a Senior Content Classifier. Classify articles as WHAT/HOW/WHY.

WHAT: News, announcements, updates
HOW: Tutorials, guides, technical explanations
WHY: Philosophy, strategy, reasoning

Respond with JSON:
{"category": "WHAT|HOW|WHY", "confidence": "High|Medium|Low", "tags": ["tag1", "tag2"]}
"""

# Initialize OpenAI client (works with OpenAI or compatible APIs)
def get_client():
    """Get configured OpenAI client (supports Moonshot/Kimi, OpenAI, etc.)"""
    api_key = os.getenv("OPENAI_API_KEY") or os.getenv("MOONSHOT_API_KEY")
    base_url = os.getenv("OPENAI_BASE_URL") or os.getenv("MOONSHOT_BASE_URL", "https://api.openai.com/v1")

    if not api_key:
        raise ValueError("Please set OPENAI_API_KEY or MOONSHOT_API_KEY environment variable")

    return OpenAI(api_key=api_key, base_url=base_url)

def classify_article(title, content, model="kimi-k2-turbo-preview"):
    """
    Classify an article using AI

    Args:
        title: Article title
        content: Full article content
        model: OpenAI model to use

    Returns:
        dict with classification results
    """
    client = get_client()

    completion = client.chat.completions.create(
        model=model,
        messages=[
            {"role": "system", "content": load_prompt()},
            {"role": "user", "content": f"Article title: {title}\n\nArticle content:\n{content[:3000]}"}  # Limit content
        ],
        temperature=0.1,
        response_format={"type": "json_object"}
    )

    result_text = completion.choices[0].message.content

    try:
        result = json.loads(result_text)
        # Validate required fields
        required_fields = ["category", "confidence", "tags"]
        if not all(field in result for field in required_fields):
            raise ValueError("Missing required fields in AI response")
        return result
    except (json.JSONDecodeError, ValueError) as e:
        print(f"❌ Failed to parse AI response: {e}")
        return {
            "reasoning": "Fallback classification",
            "category": "WHAT",
            "confidence": "Low",
            "tags": []
        }

if __name__ == "__main__":
    # Test the classifier
    test_title = "Introducing Rust at Scale: Lessons from Meta"
    test_content = "At Meta, we've been migrating critical systems to Rust..."
    result = classify_article(test_title, test_content)
    print(json.dumps(result, indent=2))
