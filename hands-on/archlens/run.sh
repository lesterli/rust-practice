#!/bin/bash
# ArchLens - One-command setup and run

set -e

echo "🚀 ArchLens Setup & Run"
echo "========================"

# Check if uv is installed
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed. Please install it first:"
    echo "   curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

# Check if .env exists
if [ ! -f .env ]; then
    echo ""
    echo "⚠️  Creating .env file for API key configuration"
    echo "Please edit .env and add your API key:"
    echo ""
    cat > .env << 'EOF'
# Option 1: OpenAI
# OPENAI_API_KEY=your-api-key-here

# Option 2: Moonshot/Kimi
# MOONSHOT_API_KEY=your-api-key-here
# MOONSHOT_BASE_URL=https://api.moonshot.cn/v1
EOF
    echo "✅ Created .env - please add your API key"
    echo ""
fi

# Check if sources.json has valid sources
if [ ! -s sources.json ] || [ $(grep -c '"url"' sources.json) -eq 0 ]; then
    echo "⚠️  sources.json is empty or invalid"
    echo "Please add RSS feed URLs to sources.json"
    exit 1
fi

# Install dependencies
echo ""
echo "📦 Installing dependencies..."
uv add feedparser trafilatura openai python-dotenv streamlit 2>/dev/null || true

# Run ingestion
echo ""
echo "🔄 Running ingestion pipeline..."
if ! uv run python ingest.py; then
    echo ""
    echo "❌ Ingestion failed. Please check:"
    echo "   1. Your API key in .env"
    echo "   2. RSS URLs in sources.json"
    echo "   3. Internet connection"
    exit 1
fi

# Start Streamlit
echo ""
echo "🌐 Starting web UI..."
echo "Opening http://localhost:8501 in 3 seconds..."
sleep 3
uv run streamlit run streamlit_app.py
