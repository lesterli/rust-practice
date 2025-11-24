#!/usr/bin/env python3
"""
Test AI Classifier on Latest RSS Post
This script fetches the latest post from an RSS feed and uses AI to categorize it.
"""

import json
import feedparser
import trafilatura
from openai import OpenAI

from dotenv import load_dotenv
import os

# Load environment variables from .env file
load_dotenv()

# Load sources configuration
with open('sources.json', 'r') as f:
    sources = json.load(f)

# Use first source for testing
source = sources[0]

print(f"\n📡 Fetching latest post from: {source['name']}")
print(f"URL: {source['url']}\n")

# Fetch and parse RSS feed
feed = feedparser.parse(source['url'])

if not feed.entries:
    print("❌ No entries found in RSS feed!")
    exit(1)

# Get the latest post
latest_post = feed.entries[0]
title = latest_post.title
url = latest_post.link
published = latest_post.get('published', 'Unknown')

print(f"📝 Title: {title}")
print(f"🔗 URL: {url}")
print(f"📅 Published: {published}")
print("\n" + "="*80 + "\n")

# Extract clean text from the article
print("🔄 Extracting article content...")
if hasattr(latest_post, 'content'):
    # Some feeds include full content
    html_content = latest_post.content[0].value
elif hasattr(latest_post, 'summary'):
    html_content = latest_post.summary
else:
    # Fallback: fetch the URL and extract content
    import urllib.request
    with urllib.request.urlopen(url) as response:
        html_content = response.read().decode('utf-8')

clean_text = trafilatura.extract(html_content, include_comments=False, include_tables=False)

if not clean_text:
    print("❌ Failed to extract content from article")
    exit(1)

print(f"✅ Extracted {len(clean_text)} characters")
print("\n📄 Article Preview (first 200 chars):")
print("-" * 80)
print(clean_text[:200] + "..." if len(clean_text) > 200 else clean_text)
print("-" * 80 + "\n")

# AI Classification Prompt
SYSTEM_PROMPT = """You are a Senior Technical Editor for an engineering publication. Your job is to classify technical blog posts for a content aggregator.

### CLASSIFICATION TAXONOMY
Classify the input text into exactly one of these three categories based on the **Primary Reader Value**:

1. **WHAT (News & Status)**
   - **Core Intent:** To inform about a temporal event.
   - **Signals:** Release notes, funding announcements, "We launched X", feature lists, changelogs.
   - **Reader Goal:** "I want to know what is new."

2. **HOW (Implementation & Practice)**
   - **Core Intent:** To teach a skill or explain a mechanism.
   - **Signals:** Code snippets, "How to", tutorials, debugging guides, benchmarks, deep-dives into internal mechanics (e.g., "How the Go scheduler works").
   - **Reader Goal:** "I want to build this" or "I want to understand how this works under the hood."

3. **WHY (Strategy & Culture)**
   - **Core Intent:** To persuade or reflect.
   - **Signals:** "Why we chose X", architectural trade-offs, post-mortems, team culture, career advice, industry predictions, hot takes/opinions.
   - **Reader Goal:** "I want to understand the decision-making process."

### TIE-BREAKING RULES
- If a post explains "How" we built it to justify "Why" we built it -> Prioritize **WHY**.
- If a post announces a feature (What) but spends 80% of the text showing code examples (How) -> Prioritize **HOW**.

### TAGGING RULES
- Extract specific technologies, languages, frameworks, or libraries.
- **Avoid** generic tags like "Technology", "Coding", "Software Engineering", "Blog".
- **Normalize** casing (e.g., use "Node.js" not "node").
- Max 5 tags.

### OUTPUT FORMAT
Respond ONLY with valid JSON. Do not include markdown formatting (```json).

{
    "reasoning": "A brief 1-sentence explanation of why you chose this category.",
    "category": "WHAT|HOW|WHY",
    "confidence": "High|Medium|Low",
    "tags": ["SpecificTool", "Language", "Framework"]
}
"""

# Call OpenAI API
# Get the API key
kimi_api_key = os.getenv("MOONSHOT_API_KEY")
kimi_base_url = os.getenv("MOONSHOT_BASE_URL")
client = OpenAI(
    api_key=kimi_api_key, # <--use MOONSHOT Kimi API Key
    base_url=kimi_base_url, # <-- replace https://api.openai.com/v1
)

print("🤖 Sending to AI for classification...")
print(f"📤 Sending {len(clean_text)} characters to OpenAI...")

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=[
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": f"Article title: {title}\n\nArticle content:\n{clean_text}"}
    ],
    temperature=0.1,
    response_format={"type": "json_object"}
)

result_text = completion.choices[0].message.content
print("\n" + "="*80)
print("🎯 AI CLASSIFICATION RESULT")
print("="*80 + "\n")

try:
    result = json.loads(result_text)
    print(json.dumps(result, indent=2))
except json.JSONDecodeError:
    print("❌ Failed to parse AI response as JSON")
    print(f"Raw response: {result_text}")

print("\n" + "="*80)
print("✅ Test Complete!")
print("="*80 + "\n")
