"""
Main ingestion pipeline for ArchLens
Fetches RSS feeds, extracts content, and classifies posts with AI
"""

import feedparser
import trafilatura
import urllib.request
from datetime import datetime
import json
from database import (
    init_database,
    load_sources,
    get_all_sources,
    check_post_exists,
    insert_post
)
from ai_classifier import classify_article

def get_entry_date(entry):
    """
    Robustly extract publication date from feedparser entry.
    Returns datetime object or None.

    Uses feedparser's built-in parsing (public API) for reliability.
    """
    # Check preferred parsed date fields (already in GMT)
    for date_field in ["published_parsed", "updated_parsed", "created_parsed"]:
        if hasattr(entry, date_field) and getattr(entry, date_field):
            return datetime(*getattr(entry, date_field)[:6])

    # Fallback: use feedparser's public date parser
    date_str = entry.get("published") or entry.get("updated") or entry.get("created")
    if date_str:
        # Use feedparser's public date parser (not the private _parse_date)
        parsed_date = feedparser.parse_date(date_str)
        if parsed_date:
            return datetime(*parsed_date[:6])

    return None

def extract_article_content(url):
    """Extract clean text from article URL"""
    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            html_content = response.read().decode('utf-8')

        clean_text = trafilatura.extract(
            html_content,
            include_comments=False,
            include_tables=False,
            output_format='txt'
        )

        return clean_text
    except Exception as e:
        print(f"⚠️ Failed to extract content from {url}: {e}")
        return None

def process_feed(source_id, source_name, feed_url):
    """Process a single RSS feed"""
    print(f"\n📡 Processing: {source_name}")
    print(f"   URL: {feed_url}")

    feed = feedparser.parse(feed_url)
    posts_added = 0
    posts_skipped = 0

    for entry in feed.entries[:3]:  # Process latest 3 posts
        url = entry.link

        # Check if already exists
        if check_post_exists(url):
            posts_skipped += 1
            continue

        # Extract post data
        title = entry.title
        author = entry.get('author', None)

        # Get publication date
        dt = get_entry_date(entry)
        published_at = dt.isoformat() if dt else None

        # Get content
        if hasattr(entry, 'content'):
            html_content = entry.content[0].value
        elif hasattr(entry, 'summary'):
            html_content = entry.summary
        else:
            # Fallback: fetch from URL
            html_content = extract_article_content(url)

        if not html_content:
            continue

        # Extract clean text
        clean_text = trafilatura.extract(html_content)
        if not clean_text:
            continue

        # AI Classification
        print(f"   🤖 Classifying: {title[:50]}...")
        classification = classify_article(title, clean_text)

        # Prepare post data
        post_data = {
            'source_id': source_id,
            'title': title,
            'url': url,
            'author': author,
            'published_at': published_at,
            'content_summary': clean_text[:500],  # Store first 500 chars
            'ai_category': classification['category'],
            'ai_confidence': classification['confidence'],
            'ai_tags': classification.get('tags', [])
        }

        # Insert into database
        insert_post(post_data)
        posts_added += 1

        print(f"   ✅ Added: {classification['category']} ({', '.join(classification.get('tags', [])[:3])})")

    print(f"   📊 Result: {posts_added} added, {posts_skipped} skipped")
    return posts_added

def run_ingestion():
    """Run full ingestion pipeline"""
    print("🚀 Starting ArchLens Ingestion Pipeline")
    print("="*80)

    # Initialize database
    init_database()
    load_sources()

    # Get all sources
    sources = get_all_sources()

    total_added = 0

    # Process each source
    for source_id, source_name, feed_url in sources:
        try:
            added = process_feed(source_id, source_name, feed_url)
            total_added += added
        except Exception as e:
            print(f"❌ Error processing {source_name}: {e}")

    print("\n" + "="*80)
    print(f"✅ Ingestion complete! Total posts added: {total_added}")
    print("="*80)

if __name__ == "__main__":
    run_ingestion()
