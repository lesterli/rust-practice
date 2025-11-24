# ArchLens

> See engineering blogs the way architects do.

ArchLens is an AI-powered content aggregator that automatically ingests engineering blog posts and categorizes them using AI. Instead of just showing you posts chronologically, it organizes them by **architectural intent**: what happened, how to build it, and why it matters.

## ✨ Features

- **AI-Powered Categorization**: Classifies posts as WHAT / HOW / WHY
- **Automatic Ingestion**: Pulls from RSS feeds automatically
- **Smart Filtering**: Search by category, source, or keywords
- **Clean Web UI**: Built with Streamlit
- **SQLite Database**: Fast, embedded database
- **Minimal Setup**: Just edit `sources.json` and run

## 🚀 Quick Start

### 1. Set up environment

```bash
cd hands-on/archlens

# Install dependencies
uv add feedparser trafilatura openai python-dotenv streamlit
```

### 2. Configure API Key

Create a `.env` file:

```bash
# API Key (choose one)
OPENAI_API_KEY=your-api-key-here
# or
MOONSHOT_API_KEY=your-api-key-here
MOONSHOT_BASE_URL=https://api.moonshot.cn/v1

# Custom AI Prompt (optional - better to edit prompts.json instead)
# ARCHLENS_SYSTEM_PROMPT="""Your custom classification prompt here"""

# OpenAI-compatible API endpoint (optional)
# OPENAI_BASE_URL=https://your-api-endpoint/v1
```

**Note:** For customizing the AI prompt, it's easier to edit `prompts.json` than using the `ARCHLENS_SYSTEM_PROMPT` environment variable.

### 3. Add RSS Sources

Edit `sources.json`:

```json
[
  { "id": "meta", "name": "Meta Engineering", "url": "https://engineering.fb.com/feed/" },
  { "id": "netflix", "name": "Netflix Tech Blog", "url": "https://netflixtechblog.com/feed" },
  { "id": "airbnb", "name": "Airbnb Engineering", "url": "https://medium.com/feed/airbnb-engineering" }
]
```

### 4. Ingest Posts

```bash
uv run python ingest.py
```

This will:
- Fetch latest 3 posts from each RSS source
- Extract and clean article content
- Classify each post with AI
- Store everything in SQLite

### 5. Launch Web UI

```bash
uv run streamlit run streamlit_app.py
```

Open your browser to `http://localhost:8501`

## 📁 Project Structure

```
archlens/
├── sources.json           # RSS feed sources configuration
├── prompts.json           # AI classification prompt templates
├── database.py           # SQLite database operations
├── ai_classifier.py      # AI categorization logic
├── ingest.py             # Main ingestion pipeline
├── streamlit_app.py      # Web UI
├── archlens.db          # SQLite database (created after first run)
└── test_ai_classifier.py # Quick test script
```

## 🔧 Usage

### Ingestion Pipeline

Run the full pipeline:
```bash
uv run python ingest.py
```

Test AI classifier on one article:
```bash
uv run python test_ai_classifier.py
```

### Web Interface

The UI provides:

**Filters (Sidebar):**
- Source selection
- Category filter (WHAT / HOW / WHY)
- Keyword search

**Post Feed (Main Area):**
- Color-coded category badges
- Tag display
- "Read Original" button to open source
- Post preview (first 300 characters)

### Database Operations

Access the database directly:

```python
from database import get_posts, get_all_sources

# Get all posts
posts = get_posts()

# Filter by category
posts = get_posts(filters={'category': 'HOW'})

# Search by keywords
posts = get_posts(filters={'keywords': 'Rust'})

# Get all sources
sources = get_all_sources()
```

## 🎯 AI Categorization

The AI classifier categorizes posts into three types:

**WHAT (News & Status)**
- Release notes, feature announcements
- Company updates, new launches
- Reader goal: "What is new?"

**HOW (Implementation & Practice)**
- Tutorials, code examples
- Technical deep-dives
- Debugging guides, best practices
- Reader goal: "How do I build this?"

**WHY (Strategy & Culture)**
- Architecture decisions, trade-offs
- Post-mortems, reflections
- Industry opinions, predictions
- Reader goal: "Why did they make this choice?"

Each classification includes:
- Confidence level (High/Medium/Low)
- Up to 5 technology tags (e.g., "Rust", "Microservices", "AI")

## 🛠️ Development

### Module Details

**database.py**
- Initializes SQLite database
- Manages sources and posts tables
- Provides query functions with filtering

**ai_classifier.py**
- Loads prompts from `prompts.json`
- Wraps OpenAI API calls
- Handles errors and fallbacks

**ingest.py**
- Fetches RSS feeds with `feedparser`
- Extracts clean text with `trafilatura`
- Calls AI classifier for each post
- Deduplicates by URL

**streamlit_app.py**
- Renders the web interface
- Displays posts with filtering
- Provides category badges and tags
- Links to original sources

**prompts.json**
- Structured prompt configuration
- Edit to customize AI classification logic
- Supports multiple prompt variants (e.g., "default", "strict", "experimental")

### Customization

Add new RSS sources:
```json
{ "id": "custom-id", "name": "Your Source", "url": "https://example.com/feed" }
```

Customize AI classification prompt:
- Edit `prompts.json` to modify the classification logic
- Or set `ARCHLENS_SYSTEM_PROMPT` in `.env` for a one-off override

Example in `prompts.json`:
```json
{
  "default": {
    "role": "Your custom role...",
    "classification_taxonomy": [...],
    "output_format": "JSON format..."
  }
}
```

Change AI model:
```python
# In ai_classifier.py or ingest.py
classification = classify_article(title, content, model="gpt-4o-mini")
```

Modify content extraction:
```python
# In ingest.py
clean_text = trafilatura.extract(
    html_content,
    include_comments=False,
    include_tables=False,
    output_format='txt'
)
```

## 📊 Database Schema

**sources**
- id (TEXT PRIMARY KEY)
- name (TEXT)
- url (TEXT)
- created_at (TIMESTAMP)

**posts**
- id (INTEGER PRIMARY KEY)
- source_id (TEXT FOREIGN KEY)
- title (TEXT)
- url (TEXT UNIQUE)
- author (TEXT)
- published_at (TIMESTAMP)
- content_summary (TEXT)
- ai_category (TEXT)
- ai_confidence (TEXT)
- ai_tags (TEXT - JSON array)
- created_at (TIMESTAMP)

## 🔄 Automation

Add to crontab for automatic ingestion:

```bash
# Run every hour
0 * * * * cd /path/to/archlens && /usr/bin/uv run python ingest.py
```

Or use systemd timer for more control.

## 🐛 Troubleshooting

**"Module not found" errors**
```bash
# Ensure you're using uv run
uv run python ingest.py
```

**AI API errors**
- Check your API key in `.env`
- Verify the model name is correct
- Check your API quota/credits

**No posts showing**
- Run `uv run python ingest.py` to populate the database
- Check RSS URLs in `sources.json` are valid
- Look for errors in the terminal output

**RSS feed errors**
- Some feeds have rate limiting
- Try reducing the number of posts fetched (change `[:3]` in ingest.py)
- Check feed URLs in a browser

## 📝 License

MIT
